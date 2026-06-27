//SPDX:MIT

//! Run jobs and maintain their state as they execute.

use std::{
    sync::{Arc, RwLock},
    thread,
};

use super::run_step::{JobRunStatus, JobStatus};
use crate::shell_lib::{
    helpers::async_signal::SignalNotice,
    structure::{
        JobRef, ScriptExit, Source,
        job::{JobDescription, JobRunnerContext},
    },
};

#[derive(Debug, Clone)]
pub enum JobRunState {
    /// The job has not yet started running.
    NeverStarted,

    /// The node is currently running.
    Running,

    /// The node has stopped execution.
    Stopped,
}

/// Information about the job's running state.
#[derive(Debug)]
pub struct JobState {
    inner: Arc<RwLock<JobStateInner>>,

    // The description is read-only, so is kept outside the inner.
    // Note that the 'runner' should *only* be used from within a thread.
    desc: Arc<JobDescription>,
}

#[derive(Debug)]
pub struct JobStateInner {
    // The state reflects a summary of other values, that must be consistent:
    //   - NeverStarted: thread is None and exit is None
    //   - Running: thread is Some and exit is None or Some
    //   - Stopped: thread is Some (if not joined yet) or None and exit is Some
    state: JobRunState,

    /// The most recent exit of the node.
    /// If the job had run and is running again, this will still reflect the last run exit state.
    exit: Option<ScriptExit>,

    /// The currently running thread for the job.
    thread: Option<thread::JoinHandle<()>>,
}

impl JobState {
    pub fn new(desc: JobDescription) -> Self {
        Self {
            inner: Arc::new(RwLock::new(JobStateInner {
                state: JobRunState::NeverStarted,
                exit: None,
                thread: None,
            })),
            desc: Arc::new(desc),
        }
    }

    pub fn get_run_status(&self) -> Result<JobRunStatus, String> {
        self.inner
            .write()
            .map_err(|e| format!("job {} status poisoned lock: {:?}", self.desc.source, e))?
            .get_run_status(&self.desc.source.source)
    }

    /// Check if this job can start right now.
    /// As long as a single thread manages the execution of new jobs,
    /// this will return a reliable result.  At worst, a running job
    /// may switch to stopped, which won't affect the caller's required
    /// behavior.
    pub fn can_start(&self) -> bool {
        match self.inner.read().expect("job poisoned").state {
            JobRunState::NeverStarted => true,
            JobRunState::Running => false,
            JobRunState::Stopped => self.desc.rerunable,
        }
    }

    /// Run the job.
    /// Will panic if `can_start` returns false, so check that first.
    pub fn run(
        &mut self,
        ctx: Box<dyn JobRunnerContext + Send>,
        signal: SignalNotice,
    ) -> Result<(), String> {
        assert!(self.can_start());
        // Further validation, so we don't lose any join status.
        let inner = self.inner.clone();
        {
            inner
                .write()
                .expect("inner job lock poisoned")
                .validate_state(&self.desc.source.source)?;
        }
        JobStateInner::start(
            inner,
            ctx,
            signal,
            self.desc.source.source.clone(),
            self.desc.clone(),
        )
    }
}

impl JobStateInner {
    /// Perform the job thread creation and run state change dance.
    pub fn start(
        u_self: Arc<RwLock<Self>>,
        ctx: Box<dyn JobRunnerContext + Send>,
        signal: SignalNotice,
        source: Source,
        job: Arc<JobDescription>,
    ) -> Result<(), String> {
        let t_self = u_self.clone();
        let t_runner = move || {
            // The "running" state is set before the thread starts, so that
            // we don't have a race condition of multiple threads running before one
            // can start.
            // The job runs outside the lock.
            let res = job.runner.run(ctx);

            // Run completed, so update the state in the lock.
            {
                let mut t_self = t_self.write().expect("poisoned lock");
                t_self.exit.insert(res);
                t_self.state = JobRunState::Stopped;

                // Send the signal after finish.
                signal.notify();
            }
        };

        // In a single lock, set the state, create the thread, and set the thread.
        let mut u_self = u_self.write().map_err(|e| format!("{}: {:?}", source, e))?;
        u_self.state = JobRunState::Running;
        u_self.thread.insert(thread::spawn(t_runner));
        Ok(())
    }

    pub fn get_run_status(&mut self, source: &Source) -> Result<JobRunStatus, String> {
        self.validate_state(source)?;
        match &self.state {
            JobRunState::NeverStarted => Ok(JobRunStatus::NeverStarted),
            JobRunState::Running => Ok(JobRunStatus::Running),
            JobRunState::Stopped => match &self.exit {
                None => Err(format!("BUG state is stopped but no exit code: {}", source)),
                Some(e) => Ok(JobRunStatus::Stopped(e.clone())),
            },
        }
    }

    /// Perform internal state consistency check.
    pub fn validate_state(&mut self, source: &Source) -> Result<(), String> {
        match &self.state {
            JobRunState::NeverStarted => {
                match self.exit {
                    None => Ok(()),
                    Some(_) => Err(format!(
                        "{}: is never started but has an exit value",
                        source
                    )),
                }?;
                match self.thread {
                    None => Ok(()),
                    Some(_) => Err(format!(
                        "{}: marked as never started but has a thread",
                        source
                    )),
                }
            }
            JobRunState::Running => {
                match &self.thread {
                    None => Err(format!("{}: marked as running but has no thread", source)),
                    Some(t) => {
                        // Poke at the thread to ensure its still running.
                        if t.is_finished() {
                            Err(format!(
                                "{}: marked as running but has a stopped thread",
                                source
                            ))
                        } else {
                            Ok(())
                        }
                    }
                }
            }
            JobRunState::Stopped => {
                match &self.exit {
                    Some(_) => Ok(()),
                    None => Err(format!(
                        "{}: marked as stopped but has no exit value",
                        source
                    )),
                }?;
                match &self.thread {
                    Some(t) => {
                        // Poke at the thread.
                        if t.is_finished() {
                            // Join up the thread to ensure no panic happened.
                            // Because this will capture any error, it should clear out the thread
                            // so this doesn't run again.
                            let t = self
                                .thread
                                .take()
                                .expect("BUG something modified the object within a lock");
                            match t.join() {
                                Ok(()) => Ok(()),
                                Err(e) => {
                                    let msg =
                                        format!("{}: encountered fatal error {:?}", source, e);
                                    self.exit.insert(ScriptExit::new(250, Some(msg.clone())));
                                    Err(msg)
                                }
                            }
                        } else {
                            Err(format!(
                                "{}: marked as stopped but has a running thread",
                                source
                            ))
                        }
                    }
                    None => Ok(()),
                }
            }
        }
    }
}

/// Manage the execution of the jobs.
pub struct JobManager {
    // The list of jobs is static
    jobs: Vec<JobState>,
}

impl JobManager {
    pub fn new(jobs: Vec<JobDescription>) -> Self {
        Self {
            jobs: jobs.into_iter().map(|j| JobState::new(j)).collect(),
        }
    }

    /// Attempt to run the job.
    /// This does not fail if the job is not in a ready-to-run state.
    /// This sends a signal when the job completes.
    pub fn start_job(
        &mut self,
        job: JobRef,
        ctx: Box<dyn JobRunnerContext + Send>,
        signal: SignalNotice,
    ) -> Result<(), String> {
        match self.jobs.get_mut(job) {
            Some(j) => {
                if j.can_start() {
                    j.run(ctx, signal)
                } else {
                    Ok(())
                }
            }
            None => Err(format!("no such job {}", job)),
        }
    }

    /// Collect all jobs' status.  The JobRef is the index in the vector.
    pub fn job_states(&mut self) -> Result<Vec<JobRunStatus>, String> {
        let mut ret = Vec::with_capacity(self.jobs.len());
        for j in &mut self.jobs {
            ret.push(j.get_run_status()?);
        }
        Ok(ret)
    }
}

impl JobStatus for JobManager {
    fn job_status(&self, job_ref: JobRef) -> Result<JobRunStatus, String> {
        match self.jobs.get(job_ref) {
            None => Ok(JobRunStatus::NotExist),
            Some(j) => j.get_run_status(),
        }
    }
}
