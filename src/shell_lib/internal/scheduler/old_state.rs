//! Maintain sequence and job state as they execute.
//!
//! Currently, this mixes the construction of the model and the execution state of the model.

use std::{
    collections::HashSet,
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::shell_lib::structure;

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
    pub state: JobRunState,

    /// The most recent exit of the node.
    /// If the job had run and is running again, this will still reflect the last run exit state.
    pub exit: Option<structure::job::ScriptExit>,

    pub desc: Arc<structure::job::JobDescription>,
}

impl JobState {
    pub fn new(desc: structure::job::JobDescription) -> Self {
        Self {
            state: JobRunState::NeverStarted,
            exit: None,
            desc: Arc::new(desc),
        }
    }

    pub fn can_start(&self) -> bool {
        match self.state {
            JobRunState::NeverStarted => true,
            JobRunState::Running => false,
            JobRunState::Stopped => self.desc.rerunable,
        }
    }
}

/// Job registration implementation for this scheduler.
pub struct JobBuilder {
    inner: Rc<JobBuilderInner>,
}

impl structure::create::JobRegistration for JobBuilder {
    fn id(&self) -> structure::job::JobRef {
        self.inner.id
    }

    fn listens(mut self, event: structure::event::EventRef) -> Self {
        self.inner.listens_to.push(event);
        self
    }

    fn with_runner(mut self, runner: Box<dyn structure::job::JobRunner + Sync + Send>) -> Self {
        self.inner.runner = Some(runner);
        self
    }

    fn with_runs_once(mut self) -> Self {
        self.inner.rerunnable = false;
        self
    }
}

#[derive(Clone, Debug)]
pub enum ThreadRunState {
    /// The job sequence has not yet started running.
    NeverStarted,

    /// The job sequence is currently running.
    Running,

    /// The job sequence has stopped execution.
    Stopped,
}

#[derive(Debug)]
pub struct ThreadState {
    state: ThreadRunState,

    /// The last waited-on job or thread execution state.
    exit: Option<structure::job::ScriptExit>,

    /// The code pointer.  Only has meaning if the run state is Running.
    active_step_index: usize,

    pub thread: structure::job::JobThreadDescription,
}

impl ThreadState {
    pub fn new(thread: structure::job::JobThreadDescription) -> Self {
        // basic buildup assertion that should be handled by the builder.
        assert!(thread.steps.len() > 0);

        Self {
            state: ThreadRunState::NeverStarted,
            exit: None,
            active_step_index: 0,
            thread,
        }
    }

    /// Determine if the thread is marked as running.
    pub fn is_running(&self) -> bool {
        self.state == ThreadRunState::Running
    }

    /// Get the result from the last job or thread that this thread waited on.
    pub fn get_exit(&self) -> Option<structure::job::ScriptExit> {
        self.exit.clone()
    }

    /// Start the thread by resetting the step index and returning the first step.
    pub fn start(&mut self) -> &structure::job::ScheduleStep {
        assert_ne!(self.state, ThreadRunState::Running);
        self.active_step_index = 0;
        self.state = ThreadRunState::Running;
        self.thread
            .steps
            .get(0)
            .expect("setup requires jobs to have at least 1 step")
    }

    /// Set the exit state for a waited-on-job or thread that just completed.
    pub fn set_exit(&mut self, waited_exit: structure::job::ScriptExit) {
        assert_eq!(self.state, ThreadRunState::Running);
        self.exit = Some(waited_exit);
    }

    /// Advance the thread's step by the given step count (may be negative).
    /// Will return None if the advancement has moved beyond the end of the steps, which
    /// implicitly means it has stopped (do NOT call stop afterwards!).
    pub fn advance(&mut self, count: i32) -> Option<&structure::job::ScheduleStep> {
        assert_eq!(self.state, ThreadRunState::Running);
        let len = i64::from(self.thread.steps.len());
        let count = i64::from(count);
        let mut next = i64::from(self.active_step_index) + count;

        if next < 0 && count > 0 {
            // overflow check
            next = len;
        }

        if next <= 0 {
            self.active_step_index = 0;
        } else {
            if next >= len {
                self.active_step_index = usize::from(len);
                self.state = ThreadRunState::Stopped;
            } else {
                self.active_step_index = usize::from(next);
            }
        }
        self.thread.steps.get(self.active_step_index)
    }

    /// Change the state of the thread to stopped.
    pub fn stop(&mut self) {
        assert_eq!(self.state, ThreadRunState::Running);
        self.state = ThreadRunState::Stopped;
    }
}

/// Keeps track of the thread details as it's built up.
struct ThreadBuilderInner {
    id: structure::job::JobThreadRef,
    source: structure::source::Resource,
    steps: Vec<structure::job::ScheduleStep>,
}

impl ThreadBuilderInner {
    pub fn new(id: structure::job::JobThreadRef, source: structure::source::Resource) -> Self {
        Self {
            id,
            source,
            steps: Vec::new(),
        }
    }

    pub fn close(&self) -> Result<(structure::job::JobThreadRef, ThreadState), String> {
        if self.steps.is_empty() {
            Err(format!("no steps set for job thread {}", self.source))
        } else if self.steps.len() > structure::job::MAX_STEPS {
            Err(format!(
                "too many steps ({}, maxiumum {}) set for job thread {}",
                self.steps.len(),
                structure::job::MAX_STEPS,
                self.source
            ))
        } else {
            Ok((
                self.id,
                ThreadState::new(structure::job::JobThreadDescription {
                    source: self.source,
                    steps: self.steps,
                }),
            ))
        }
    }
}

/// JobThreadRegistration implementation for this scheduler.
pub struct ThreadBuilder {
    inner: Rc<ThreadBuilderInner>,
}

impl structure::create::JobThreadRegistration for ThreadBuilder {
    fn id(&self) -> structure::job::JobThreadRef {
        self.inner.id
    }

    fn with_steps(mut self, steps: &[structure::job::ScheduleStep]) -> Self {
        self.inner.steps = steps.into();
        self
    }
}

/// Manages the initialization of the scheduler state.
/// Like all builders, is not thread safe.
pub struct SchedulerBuilder {
    jobs: Vec<Rc<JobBuilderInner>>,
    threads: Vec<Rc<ThreadBuilderInner>>,
    events: super::eventbus::BaseEventRegistrar,
}

impl structure::create::EventRegistrar for SchedulerBuilder {
    fn add_event(&mut self, name: &str) -> structure::event::EventDesc {
        self.events.add_event(name)
    }
}

impl structure::create::JobRegistrar<JobBuilder> for SchedulerBuilder {
    fn add_job(&mut self, source: structure::source::Resource) -> JobBuilder {
        let id = self.jobs.len();
        let inner = Rc::new(JobBuilderInner::new(id, source));
        self.jobs.push(inner.clone());
        JobBuilder { inner }
    }
}

impl structure::create::JobThreadRegistrar<ThreadBuilder> for SchedulerBuilder {
    fn add_job_thread(&mut self, source: structure::source::Resource) -> ThreadBuilder {
        let id = self.threads.len();
        let inner = Rc::new(ThreadBuilderInner::new(id, source));
        self.threads.push(inner.clone());
        ThreadBuilder { inner }
    }
}

impl SchedulerBuilder {
    pub fn close(self) -> Result<GlobalState, Vec<String>> {
        let mut errs = Vec::new();
        let mut jobs = Vec::with_capacity(self.jobs.len());
        let mut threads = Vec::with_capacity(self.threads.len());
        let mut bus = self.events.close();
        for job in self.jobs {
            match job.close() {
                Ok(v) => {
                    if jobs.len() != v.0 {
                        errs.push(format!(
                            "BUG created job index out of order ({} for {})",
                            v.0, v.1.source
                        ))
                    } else {
                        jobs.push(RwLock::new(v.2));
                        bus.add_event_listeners(v.0, v.1);
                    }
                }
                Err(e) => {
                    errs.push(e);
                }
            }
        }
        for thread in self.threads {
            match thread.close() {
                Ok(v) => {
                    if threads.len() != v.0 {
                        errs.push(format!(
                            "BUG created thread index out of order ({} for {})",
                            v.0, v.1.source
                        ))
                    } else {
                        threads.push(RwLock::new(v.1));
                    }
                }
                Err(e) => {
                    errs.push(e);
                }
            }
        }

        if errs.is_empty() {
            Ok(GlobalState {
                jobs,
                threads,
                active_jobs: RwLock::new(HashSet::new()),
                active_threads: RwLock::new(HashSet::new()),
                bus,
            })
        } else {
            Err(errs)
        }
    }
}

/// The global event + job + thread state setup.  It ties everything together with the
/// right business logic.
pub struct GlobalState {
    /// The list of jobs is a static list, but their internal state changes.
    jobs: Vec<RwLock<JobState>>,

    /// The list of threads is a static list, but their internal state changes.
    threads: Vec<RwLock<ThreadState>>,

    active_jobs: RwLock<HashSet<structure::job::JobRef>>,
    active_threads: RwLock<HashSet<structure::job::JobThreadRef>>,

    bus: super::eventbus::EventBus,
}

impl GlobalState {
    /// Attempt to mark the given job as started.
    /// This will return None if the job is already running, or has already run but is not rerunnable.
    pub fn start_job(
        &self,
        job: structure::job::JobRef,
    ) -> Result<Option<Arc<structure::job::JobDescription>>, String> {
        match self.jobs.get(job) {
            Some(j) => {
                let j = j
                    .get_mut()
                    .map_err(|e| format!("job {} lock poisoned: {}", job, e))?;
                if j.can_start() {
                    self.active_jobs
                        .get_mut()
                        .map_err(|e| format!("active job lock poisoned: {}", e))?
                        .insert(job);
                    j.state = JobRunState::Running;
                    Ok(Some(j.desc.clone()))
                } else {
                    Ok(None)
                }
            }
            None => Err(format!("no such job {}", job)),
        }
    }

    /// Called when a job's run function exits.  This will return the list of job threads that
    /// have an active listener for the job to stop.
    pub fn job_stopped(
        &self,
        job: structure::job::JobRef,
        exit: structure::job::ScriptExit,
    ) -> Result<Vec<structure::job::JobThreadRef>, String> {
        match self.jobs.get(job) {
            Some(j) => {
                let j = j
                    .get_mut()
                    .map_err(|e| format!("job {} lock poisoned: {}", job, e))?;
                if j.state == JobRunState::Running {
                    j.state = JobRunState::Stopped;
                    j.exit = Some(exit);
                    self.active_jobs
                        .get_mut()
                        .map_err(|e| format!("active job lock poisoned: {}", e))
                        .remove(job);
                    Ok(self.bus.on_job_exit(job))
                } else {
                    Err(format!("job {} not running", job))
                }
            }
            None => Err(format!("no such job {}", job)),
        }
    }

    pub fn get_stopped_job_exit(
        &self,
        job: structure::job::JobRef,
    ) -> Result<Option<structure::job::ScriptExit>, String> {
        match self.jobs.get(job) {
            Some(j) => {
                let j = j
                    .get_mut()
                    .map_err(|e| format!("job {} lock poisoned: {}", job, e))?;
                if j.state == JobRunState::Stopped {
                    Ok(j.exit.clone())
                } else {
                    Ok(None)
                }
            }
            None => Err(format!("no such job {}", job)),
        }
    }

    /// Get the last job exit state.  It does not return information on whether the job is running or not.
    /// If the job has never started, then it returns None.
    pub fn get_job_exit(
        &self,
        job: structure::job::JobRef,
    ) -> Result<Option<structure::job::ScriptExit>, String> {
        match self.jobs.get(job) {
            Some(j) => {
                let j = j
                    .get_mut()
                    .map_err(|e| format!("job {} lock poisoned: {}", job, e))?;
                Ok(j.exit.clone())
            }
            None => Err(format!("no such job {}", job)),
        }
    }

    /// Call when something sends an event to the event bus to retrieve all alive jobs that listen to it.
    /// Only jobs actively running have their event listener called.
    pub fn on_event(
        &self,
        event: structure::event::EventRef,
    ) -> Result<Vec<structure::job::JobRef>, String> {
        // This needs to get the jobs listening to the event, then cross-reference those to the
        // active jobs.
        let jobs = HashSet::from(self.bus.on_event(event));
        let active = self
            .active_jobs
            .get()
            .map_err(|e| format!("active job lock poisoned: {}", e));
        Ok(jobs.difference(active).collect())
    }

    /// Get the list of all jobs actively running.
    pub fn get_active_jobs(&self) -> Result<Vec<structure::job::JobRef>, String> {
        Ok(self
            .active_jobs
            .get()
            .map_err(|e| format!("active job lock poisoned: {}", e))?
            .clone())
    }

    /// Trigger the state change to start a thread.
    /// Returns the first step in the thread if it can start, or None if it is actively running.
    pub fn start_thread(
        &mut self,
        thread: structure::job::JobThreadRef,
    ) -> Result<Option<structure::job::ScheduleStep>, String> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e))?;
                if t.is_running() {
                    Ok(None)
                } else {
                    self.active_threads
                        .get_mut()
                        .map_err(|e| format!("active job thread lock poisoned: {}", e))?
                        .insert(thread);
                    Ok(Some(t.start().clone()))
                }
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    /// Mark a thread as stopped.
    /// The thread does not test whether any jobs still run, because threads can share jobs between them.
    pub fn thread_stopped(&mut self, thread: structure::job::JobThreadRef) -> Result<(), String> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e))?;
                if t.is_running() {
                    t.stop();
                    self.active_threads
                        .get_mut()
                        .map_err(|e| format!("active job thread lock poisoned: {}", e))
                        .remove(thread);
                    Ok(())
                } else {
                    Err(format!("job thread {} not running", thread))
                }
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    pub fn get_stopped_thread_exit(
        &self,
        thread: structure::job::JobThreadRef,
    ) -> Result<Option<structure::job::ScriptExit>, String> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e))?;
                if t.is_running() {
                    Ok(None)
                } else {
                    Ok(t.get_exit())
                }
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    /// Get the last thread exit status.  If the thread has never run, then this will return None,
    /// otherwise it will return the last set exit status, even if the thread is running again.
    pub fn get_thread_exit(
        &self,
        thread: structure::job::JobThreadRef,
    ) -> Result<Option<structure::job::ScriptExit>, String> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e))?;
                Ok(t.get_exit())
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    /// After a thread waits on a job or a thread, call this to set its exit status.
    pub fn set_thread_exit(
        &self,
        thread: structure::job::JobThreadRef,
        exit: structure::job::ScriptExit,
    ) -> Result<(), String> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e));
                t.set_exit(exit);
                Ok(())
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    /// Advance the thread by the count number of steps.  If the thread has reached the end, then this
    /// returns None but does not change the running state.
    pub fn advance_thread(
        &mut self,
        thread: structure::job::JobThreadRef,
        count: structure::job::StepCount,
    ) -> Result<Option<structure::job::ScheduleStep>> {
        match self.threads.get(thread) {
            Some(t) => {
                let t = t
                    .get_mut()
                    .map_err(|e| format!("job thread {} lock poisoned: {}", thread, e));
                Ok(t.advance(count))
            }
            None => Err(format!("no such job thread {}", thread)),
        }
    }

    pub fn get_active_threads(&self) -> Result<Vec<structure::job::JobThreadRef, String>> {
        Ok(self
            .active_threads
            .get()
            .map_err(|e| format!("active job thread lock poisoned: {}", e))?
            .clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThreadRunState {
    /// The job sequence has not yet started running.
    NeverStarted,

    /// The job sequence is currently running.
    /// Note that, if the job sequence has run before, its exit code is not available,
    /// because it can only be accessed at stop time through the wait request.
    Running,

    /// The job sequence has finished running.
    Finished(ScriptExit),
}

/// High level scheduler interface for job management.
pub trait JobManager {
    /// Start a job sequence.
    /// Does nothing if the job is actively running.
    fn start_job_sequence(&self, seq_ref: JobThreadRef) -> Result<(), String>;

    /// Get the current state of the job sequence.
    fn get_job_sequence_state(&self, seq_ref: JobThreadRef) -> JobThreadRunState;

    /// Wait for a job sequence to finish executing.
    /// Returns immediately if the sequence has not started.
    fn wait_for_job_sequence(
        &self,
        seq_ref: JobThreadRef,
        timeout: Option<std::time::Duration>,
    ) -> Result<Option<ExitCode>, String>;
}

/// The full job scheduler contract - start and wait for job sequences, and send events into the active jobs.
pub trait JobScheduler: JobManager + JobRunnerContext {}
