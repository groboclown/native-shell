//SPDX:MIT

//! Defines the job that the compiler constructs to allow for interaction with the scheduler.
//! These loosely map to the actions and execution of the modules in the script.
//!

use std::collections::HashSet;
use std::ops::Range;
use std::sync::mpsc::SendError;
use std::sync::{Arc, PoisonError};

use super::event::{EventPayload, EventRef};
use super::source::Resource;

/// A script-wide unique reference to a specific job.
pub type JobRef = usize;
pub type ExitCode = i32;

/// An exit from a job step or thread.
#[derive(Clone, Debug)]
pub struct ScriptExit {
    pub code: ExitCode,
    pub message: Option<String>,
}

impl ScriptExit {
    pub fn new(code: ExitCode, message: Option<String>) -> Self {
        Self { code, message }
    }

    /// Join the slice of ScriptExit into a single ScriptExit.
    pub fn join_slice(exits: &[ScriptExit]) -> ScriptExit {
        if exits.len() <= 0 {
            ScriptExit {
                code: 0,
                message: None,
            }
        } else if exits.len() == 1 {
            exits[0].clone()
        } else {
            let mut m_first = true;
            let mut msg = String::new();
            let mut err_count: ExitCode = 0;
            for e in exits {
                if e.code != 0 {
                    err_count += 1;
                }
                if let Some(m) = &e.message {
                    if m_first {
                        m_first = false;
                    } else {
                        msg.push('\n');
                    }
                    msg.push_str(m.as_str());
                }
            }
            if m_first {
                ScriptExit {
                    code: err_count,
                    message: None,
                }
            } else {
                ScriptExit {
                    code: err_count,
                    message: Some(msg),
                }
            }
        }
    }
}

impl From<ExitCode> for ScriptExit {
    fn from(value: ExitCode) -> Self {
        Self {
            code: value,
            message: None,
        }
    }
}

impl From<&str> for ScriptExit {
    fn from(value: &str) -> Self {
        Self {
            code: 1,
            message: Some(value.to_string()),
        }
    }
}

/// Converts an String error into a ScriptExit.
/// Because this conversion happens on errors, it will set the code to non-zero.
impl From<String> for ScriptExit {
    fn from(value: String) -> Self {
        Self {
            code: 1,
            message: Some(value),
        }
    }
}

/// Converts an Option<String> error into a ScriptExit.
/// Because this conversion happens on errors, it will set the code to non-zero.
impl From<Option<String>> for ScriptExit {
    fn from(value: Option<String>) -> Self {
        Self {
            code: 1,
            message: value,
        }
    }
}

/// Convert a Lock poison error to a ScriptExit.
impl<T> From<PoisonError<T>> for ScriptExit {
    fn from(value: PoisonError<T>) -> Self {
        Self {
            code: 254,
            message: Some(format!("lock poisoned: {:?}", value)),
        }
    }
}

/// Convert a channel send request error into a ScriptExit.
impl<T> From<SendError<T>> for ScriptExit {
    fn from(value: SendError<T>) -> Self {
        Self {
            code: 254,
            message: Some(format!("async communication failed: {:?}", value)),
        }
    }
}

/// An abstract job execution handler.
/// Note that the event system, to pass events to the JobRunner instances, must exist outside
/// the base runtime handling.
/// By its nature, a JobRunner is multi-threaded.
pub trait JobRunner {
    /// Execute the job.
    fn run(&self, context: Box<dyn JobRunnerContext>) -> ScriptExit;
}

/// Context sent to the job runner to allow it to have limited interaction with the scheduler.
/// Note that the job itself should not manage job state (start and wait), as that's handled by the job sequences.
/// If a job needs to handle things like conditions, then use the exit code behavior.
pub trait JobRunnerContext {
    /// Send an event to the event group.
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit>;
}

/// A description of a job that is used to run behavior in the scheduler.
/// These are created by the script compiler and executed by the engine.
/// A job is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
pub struct JobDescription {
    /// The source code or script associated with the job; for debugging.
    pub source: Resource,

    /// If true, spawning the exact same job a second time (after it has completed a previous execution)
    /// will cause the job to run another time.  If false, the job may run at most one time per script
    /// execution.
    pub rerunable: bool,

    /// Set of event IDs that the job, while running, receives events.
    pub listens_to: HashSet<EventRef>,

    /// Job handler.
    pub runner: Box<dyn JobRunner + Sync + Send>,
}

impl std::fmt::Debug for JobDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobDescription")
            .field("source", &self.source)
            .field("rerunable", &self.rerunable)
            .field("listens_to", &self.listens_to)
            .finish()
    }
}

/// A read-only store of all registered job descriptions for a script.
///
/// The JobStore may be constructed through either the JobRegistrar, which
/// gives a programmatic approach to creating jobs with correctly generated
/// JobRef IDs for handling with other constructs, or through an explicit
/// creation that requires the caller to keep track of JobRef IDs as the
/// index in the passed-in list.
pub struct JobStore {
    store: Vec<Arc<JobDescription>>,
}

impl JobStore {
    /// Create an explicit job store.  Only use this if the caller keeps
    /// track of JobRef == index in the vector; otherwise, build the JobStore
    /// through the JobRegistrar.
    pub fn new_explicit(jobs: Vec<JobDescription>) -> Self {
        Self {
            store: jobs.into_iter().map(|j| Arc::new(j)).collect(),
        }
    }

    /// Get the number of jobs stored in the store.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Get the list of job references in the store.
    pub fn job_refs(&self) -> Range<JobRef> {
        0..(self.store.len() - 1)
    }

    pub fn job(&self, j_ref: JobRef) -> Option<Arc<JobDescription>> {
        self.store.get(j_ref).map(|j| j.clone())
    }
}

/// Store for building jobs.
///
/// Jobs do not have mechanisms for inter-job references.  The only case for this exists with
/// events, and, for those, the builder should construct its own mechanism for unique inter-job
/// event names.
///
/// The job registration, like all setup stage structures, does not allow for asynchronous construction.
pub struct JobRegistrar {
    store: Vec<Arc<JobDescription>>,
}

impl JobRegistrar {
    pub fn new() -> Self {
        JobRegistrar { store: Vec::new() }
    }

    /// Register a new job for eventual construction.
    pub fn register(&mut self, job: JobDescription) -> JobRef {
        let j_ref = self.store.len();
        self.store.push(Arc::new(job));
        j_ref
    }

    /// Complete registration and create the read-only store.
    pub fn close(self) -> Result<JobStore, String> {
        Ok(JobStore { store: self.store })
    }
}

/// Allows for building up a job description incrementally.
pub struct JobBuilder {
    source: Resource,
    rerunnable: bool,
    listens_to: HashSet<EventRef>,
    runner: Option<Box<dyn JobRunner + Sync + Send>>,
}

impl From<Resource> for JobBuilder {
    fn from(source: Resource) -> Self {
        Self {
            source,
            rerunnable: true,
            listens_to: HashSet::new(),
            runner: None,
        }
    }
}

impl JobBuilder {
    pub fn new(name: &str, kind: &str, file: &str, line: i64, column: i64) -> Self {
        Resource::new(name, kind, file, line, column).into()
    }

    /// Add an event reference that this job responds to.
    /// The job will only react to events while the job is running.
    pub fn add_listens_to(&mut self, event: EventRef) {
        assert!(!self.listens_to.contains(&event));
        self.listens_to.insert(event);
    }

    /// Add an event reference that this job responds to.
    /// The job will only react to events while the job is running.
    pub fn with_listens_to(mut self, event: EventRef) -> Self {
        self.add_listens_to(event);
        self
    }

    /// Set the runner for the job execution.
    pub fn set_runner(&mut self, runner: Box<dyn JobRunner + Sync + Send>) {
        self.runner.insert(runner);
    }

    /// Set the runner for the job execution.
    pub fn with_runner(mut self, runner: Box<dyn JobRunner + Sync + Send>) -> Self {
        self.set_runner(runner);
        self
    }

    /// Mark the runner as only runnable once.  Requests to start it a second time
    /// will just respond with the last execution's exit status.
    pub fn set_runs_once(&mut self) {
        self.rerunnable = false;
    }

    /// Mark the runner as only runnable once.  Requests to start it a second time
    /// will just respond with the last execution's exit status.
    pub fn with_runs_once(mut self) -> Self {
        self.set_runs_once();
        self
    }

    /// Sets the rerunnable mode of the job.
    pub fn set_rerunnable(&mut self, rerunnable: bool) {
        self.rerunnable = rerunnable;
    }

    /// Sets the rerunnable mode of the job.
    pub fn with_rerunnable(mut self, rerunnable: bool) -> Self {
        self.set_rerunnable(rerunnable);
        self
    }

    // Close off the builder, performing validation checks.
    pub fn close(mut self) -> Result<JobDescription, String> {
        match self.runner.take() {
            Some(runner) => Ok(JobDescription {
                source: self.source.clone(),
                listens_to: self.listens_to.clone(),
                rerunable: self.rerunnable,
                runner,
            }),
            None => Err(format!("Did not set runner for {}", self.source)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_lib::structure::event::{EventKind, EventRegistrar};

    #[test]
    fn test_job_buildup() {
        let (e0, e1) = {
            let mut reg = EventRegistrar::new();
            let e0 = reg.add_event("abort", &EventKind::Signal);
            let e1 = reg.add_event("sig", &EventKind::Signal);
            reg.close();
            (e0, e1)
        };

        let mut reg = JobRegistrar::new();
        let j0_ref = reg.register(
            JobBuilder::new("j0", "job", "s.sh", 1, 1)
                .with_listens_to(e0)
                .with_listens_to(e1)
                .with_runner(Box::new(SampleJob {}))
                .close()
                .expect("should have built j0 correctly"),
        );
        assert_eq!(0, j0_ref);

        let j1_ref = reg.register(
            JobBuilder::new("j1", "job", "s.sh", 2, 1)
                .with_runner(Box::new(SampleJob {}))
                .close()
                .expect("should have built j1 correctly"),
        );
        assert_eq!(1, j1_ref);
        let store = reg.close().expect("registration should be good.");

        let j0 = store.job(j0_ref).expect("j0 should be registered");
        assert_eq!("j0".to_string(), j0.source.name);

        let j1 = store.job(j1_ref).expect("j1 should be registered");
        assert_eq!("j0".to_string(), j1.source.name);
    }

    #[test]
    fn test_job_not_configured() {
        assert_eq!(
            "".to_string(),
            JobBuilder::new("j0", "job", "s.sh", 1, 1)
                .close()
                .expect_err("did not report setup error")
        );
    }

    struct SampleJob {}
    impl JobRunner for SampleJob {
        fn run(&self, _: Box<dyn JobRunnerContext>) -> ScriptExit {
            panic!("not runnable");
        }
    }
}
