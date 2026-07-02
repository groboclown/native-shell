//SPDX:MIT

use std::collections::HashSet;
use std::time;

use super::run_step::{JobRunStatus, LoopState, Requests};
use crate::shell_lib::helpers::async_signal;
use crate::shell_lib::helpers::se_collect::ScriptExitCollector;
use crate::shell_lib::structure::job::JobDescription;
use crate::shell_lib::structure::mod_impl::CommandHandler;
use crate::shell_lib::structure::thread::{EventCollection, ThreadStore};
use crate::shell_lib::structure::{JobRef, ScriptExit, ThreadRef};

use super::eventbus::JobEventBus;
use super::run_job::JobManager;
use super::run_step::ThreadManager;

/// Runs a script according to the standard.
/// While the jobs run in parallel, the functions here must be run from the same thread.
pub struct ScheduleRunner {
    signal_notice: async_signal::SignalNotice,
    signal_wait: async_signal::SignalWait,
    jobs: JobManager,
    steps: ThreadManager,
    events: JobEventBus,
    previously_running_jobs: HashSet<JobRef>,
    started: bool,
}

impl ScheduleRunner {
    pub fn new(
        start_threads: Vec<ThreadRef>,
        jobs: Vec<JobDescription>,
        threads: ThreadStore,
        events: JobEventBus,
    ) -> Result<Self, String> {
        let (signal_notice, signal_wait) = async_signal::signal();
        Ok(Self {
            signal_notice,
            signal_wait,
            jobs: JobManager::new(jobs),
            steps: ThreadManager::new(start_threads, threads)?,
            events,
            previously_running_jobs: HashSet::new(),
            started: false,
        })
    }

    /// Perform the full run process.
    /// This runs the command's start,
    /// executes the 'run',
    /// runs the command's on_exit,
    /// waits for jobs to finish (up to the initial timeout),
    /// then runs the command's on_shutdown.
    pub fn full_run(
        &mut self,
        timeout: time::Duration,
        command: &dyn CommandHandler,
    ) -> ScriptExit {
        let final_end = time::Instant::now() + timeout;

        {
            let ctx = Box::new(self.events.as_async_sender(self.signal_notice.clone()));
            let exit = match command.start(ctx) {
                Ok(e) => e,
                Err(e) => e,
            };
            if exit.code != 0 {
                return exit;
            }
        }

        let exit = self.run(timeout);
        let mut col = ScriptExitCollector::new();
        col.add(&exit);
        {
            let ctx = Box::new(self.events.as_async_sender(self.signal_notice.clone()));
            if let Err(e) = command.on_exit(ctx, exit) {
                col.add(&ScriptExit::new(1, Some(e)));
            }
        }
        match self.wait_for_jobs(final_end - time::Instant::now()) {
            Ok(exit) => col.add(&exit),
            Err(msg) => col.add(&msg.into()),
        }
        if let Err(e) = command.on_shutdown() {
            col.add(&ScriptExit::new(1, Some(e)));
        }
        col.close()
    }

    /// Run the threads to completion, or until the timeout.
    pub fn run(&mut self, timeout: time::Duration) -> ScriptExit {
        assert_eq!(self.started, false);
        self.started = true;

        let end_time = time::Instant::now() + timeout;

        while time::Instant::now() < end_time {
            // If the script comes to an abnormal *or* normal exit, then it returns an error.
            match self.step() {
                Err(e) => {
                    return e;
                }
                Ok(s) => match s {
                    StepResult::Wait => {
                        let dur = end_time - time::Instant::now();
                        if dur > time::Duration::ZERO {
                            self.signal_wait.wait_for(dur);
                        }
                    }

                    // Keep looping
                    StepResult::Active => (),
                },
            }
        }

        // Timed out.
        ScriptExit::new(251, Some("script timed out".into()))
    }

    /// Wait for all still-running jobs to finish.
    /// Optionally call after the run completes.  It will generate an error if,
    /// when the timeout ends, any job is still running.
    pub fn wait_for_jobs(&mut self, timeout: time::Duration) -> Result<ScriptExit, String> {
        let end_time = time::Instant::now() + timeout;
        loop {
            let mut all_stopped = true;
            let mut coll = ScriptExitCollector::new();
            for s in self.jobs.job_states()? {
                match s {
                    JobRunStatus::NotExist => panic!("BUG should never happen"),
                    JobRunStatus::NeverStarted => (),
                    JobRunStatus::Running => {
                        // No need to check further.
                        all_stopped = false;
                        break;
                    }
                    JobRunStatus::Stopped(script_exit) => {
                        coll.add(&script_exit);
                    }
                }
            }
            if all_stopped {
                return Ok(coll.close());
            }
            let dur = end_time - time::Instant::now();
            if dur <= time::Duration::ZERO {
                return Err("timed out waiting for jobs to end".into());
            }

            // Can still have something stop.
            self.signal_wait.wait_for(dur);
        }
    }

    /// Run a single step through the whole process.
    /// The StepResult does not contain an is-complete status.  Instead,
    /// that's returned as the "error" result.
    fn step(&mut self) -> Result<StepResult, ScriptExit> {
        // 1. Collect events and stopped jobs.
        let r = Requests {
            jobs_ended: self.collect_stopped_jobs()?,
            new_events: self.events.collect_and_handle()?,
        };

        // 2. Perform the thread steps.
        let resp = self.steps.run_step(&r, &self.jobs)?;

        // 3. If the step caused the threads to finish, then return early.
        let ret = match resp.state {
            LoopState::Stopped(script_exit) => return Err(script_exit),
            LoopState::Waiting => StepResult::Wait,
            LoopState::Active => StepResult::Active,
        };

        // 4. Handle the newly spawned things from the threads.
        for j in resp.spawned_jobs {
            let ctx = Box::new(self.events.as_async_sender(self.signal_notice.clone()));
            self.jobs.start_job(j, ctx, self.signal_notice.clone())?;
        }
        if !resp.generated_events.is_empty() {
            for e in resp.generated_events {
                self.events.add_event(e.0, e.1)?;
            }
            self.events.collect_and_handle()?;
        }

        Ok(ret)
    }

    /// Collect all the jobs whose state has changed from running to stopped.
    /// This also updates the history so it reflects the current status.
    fn collect_stopped_jobs(&mut self) -> Result<Vec<(JobRef, ScriptExit)>, String> {
        let mut stopped = Vec::new();
        let mut running = HashSet::new();
        let states = self.jobs.job_states()?;
        for (job_ref, state) in states.iter().enumerate() {
            match state {
                JobRunStatus::NotExist => panic!("BUG should never happen"),
                JobRunStatus::NeverStarted => (),
                JobRunStatus::Running => {
                    running.insert(job_ref);
                }
                JobRunStatus::Stopped(script_exit) => {
                    if self.previously_running_jobs.contains(&job_ref) {
                        stopped.push((job_ref, script_exit.clone()))
                    }
                }
            }
        }
        self.previously_running_jobs = running;
        Ok(stopped)
    }
}

enum StepResult {
    Wait,
    Active,
}
