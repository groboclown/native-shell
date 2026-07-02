//SPDX:MIT

//! Defines the job that the compiler constructs to allow for interaction with the scheduler.
//! These loosely map to the actions and execution of the modules in the script.

use std::cell::RefCell;
use std::rc::Rc;

use super::event::{Event, EventPayload, EventRef};
use super::job::{ExitCode, JobRef, JobRunnerContext, ScriptExit};
use super::source::Resource;

pub type ThreadRef = usize;

pub type StepCount = i16;
pub const MAX_STEPS: usize = StepCount::MAX as usize;

/// A single step in a thread.
#[derive(Clone, Debug)]
pub enum ScheduleStep {
    /// Send an event to the event group.
    SendEvent(EventRef, EventPayload),

    /// Wait for a named event to happen.
    /// If the payload exists, then the payload must match along with the ref to count as
    /// a match and allow advancement.
    WaitForEvent(EventRef, Option<EventPayload>),

    /// Spawn (or rerun) the job in the scheduler.  Does nothing if the job is already running.
    /// The job has a single run state, but it may be invoked through different job references.
    /// A single job may also be active in several sequences.
    /// This does not wait for the job to finish.
    SpawnJob(JobRef),

    /// Wait for a job to finish executing.  This will block until the job's execution exits.
    /// The first parameter is the job reference.
    /// The second is how to handle the job's exit code, or if it has never started.
    WaitForJob(JobRef, ExitBehavior),

    /// Spawn a job thread.  This will run the thread in parallel to the current thread.
    SpawnThread(ThreadRef),

    /// Wait for a job sequence to finish executing.  This will block until the sequence's execution exits.
    WaitForThread(ThreadRef, ExitBehavior),

    /// Wait for all the jobs and job threads to finish.
    /// This has additional logic to join together the exits -
    /// the exit code is the sum of failed items, and the messages are joined.
    /// Note that this adds quite a bit of complexity to any thread runner, but the
    /// use case allows for capturing all errors for a web of spawned jobs.
    WaitForAll(Vec<JobRef>, Vec<ThreadRef>, ExitBehavior),

    /// For testing and possible step precision.
    Noop,
}

/// How to handle the job sequence execution after a step completes.
#[derive(Clone, Debug)]
pub enum OnExitBehavior {
    /// Jump to another action in the sequence, relative to the current action index.
    /// If the evaluated index is before 0, then this goes to the start of the thread's steps.  If
    /// the evaluated index is after the last step, then this stops the thread using the last job's
    /// exit code.
    JumpBy(StepCount),

    /// Skip all remaining actions in the sequence.
    /// The final exit code will be the job's exit code, or `ExitCode::MIN` if the job has never started.
    /// If the error is given and the job did not error, then the thread stops with this error.
    StopThread(Option<ScriptExit>),

    /// Abort the script execution, generating the given error message.
    /// This turns into an ABORT_SCRIPT_EVENT event + set the exit code.  If the exit code isn't given,
    /// then this reuses the previous exit code (or 1 if that was 0).
    AbortScript(Option<ScriptExit>),
}

/// Common behavior for standard instruction advancement.
pub const NEXT_INSTRUCTION: OnExitBehavior = OnExitBehavior::JumpBy(1);

/// Covers a range of exit codes.
#[derive(Clone, Debug)]
pub enum ExitCodeRange {
    /// A single exit code.
    Exact(ExitCode),

    /// A range of exit codes, inclusive.
    Range(ExitCode, ExitCode),

    /// All exit codes at or above a number.
    AtOrAbove(ExitCode),

    /// All exit codes at or below a number.
    AtOrBelow(ExitCode),
}

impl ExitCodeRange {
    pub fn new(min: Option<ExitCode>, max: Option<ExitCode>) -> Self {
        match (min, max) {
            (Some(min), Some(max)) if min <= max => ExitCodeRange::Range(min, max),
            (Some(min), Some(max)) if min == max => ExitCodeRange::Exact(min),
            (Some(_), Some(_)) => panic!("Invalid exit code range: min > max"),
            (Some(min), None) => ExitCodeRange::AtOrAbove(min),
            (None, Some(max)) => ExitCodeRange::AtOrBelow(max),
            (None, None) => panic!(
                "Cannot create an exit code range with no bounds; use default in behavior instead"
            ),
        }
    }

    /// Check if the given exit code is within the range.
    pub fn contains(&self, code: ExitCode) -> bool {
        match self {
            ExitCodeRange::Exact(c) => *c == code,
            ExitCodeRange::Range(start, end) => *start <= code && code <= *end,
            ExitCodeRange::AtOrAbove(start) => code >= *start,
            ExitCodeRange::AtOrBelow(end) => code <= *end,
        }
    }
}

/// How to handle the job sequence execution after a job completes.
#[derive(Clone, Debug)]
pub struct ExitCodeRangeBehavior {
    /// The exit code range covered by this behavior.
    pub code: ExitCodeRange,

    /// The behavior to take based on the exit code of the job.
    pub behavior: OnExitBehavior,
}

impl ExitCodeRangeBehavior {
    pub fn new(code: ExitCodeRange, behavior: OnExitBehavior) -> Self {
        Self { code, behavior }
    }

    /// Check if the given exit code is within the range and return the behavior.
    pub fn matches(&self, code: ExitCode) -> Option<OnExitBehavior> {
        if self.code.contains(code) {
            Some(self.behavior.clone())
        } else {
            None
        }
    }
}

/// Defines how to handle all exit codes returned by a job or thread.
///
/// To simulate a "on success to X and on failure do Y", set a
/// `exit_code_behavior` for exact == 0 for on-success, and
/// `default_behavior` for on-failure.
#[derive(Clone, Debug)]
pub struct ExitBehavior {
    /// The default behavior if the job has never started.
    pub never_started: OnExitBehavior,

    /// The behaviors based on the exit code of the job.
    pub exit_code_behaviors: Vec<ExitCodeRangeBehavior>,

    /// The behavior to use if none of the exit code behaviors match.
    pub default_behavior: OnExitBehavior,
}

impl ExitBehavior {
    /// Get the behavior for the given exit code.
    pub fn behavior_for(&self, code: Option<ExitCode>) -> OnExitBehavior {
        match code {
            None => self.never_started.clone(),
            Some(code) => {
                for behavior in &self.exit_code_behaviors {
                    if let Some(b) = behavior.matches(code) {
                        return b.clone();
                    }
                }

                self.default_behavior.clone()
            }
        }
    }

    /// Create the most common behavior - move to the next step and ignore
    /// any exit code; the equivalent of ';' in a POSIX shell script.
    pub fn new_ignore_next() -> Self {
        Self {
            never_started: NEXT_INSTRUCTION.clone(),
            exit_code_behaviors: Vec::new(),
            default_behavior: NEXT_INSTRUCTION.clone(),
        }
    }

    /// Create a behavior that aborts the script on a job error (non-zero exit code),
    /// otherwise moves to the next step.
    pub fn new_abort_on_error(exit: Option<ScriptExit>) -> Self {
        Self {
            never_started: OnExitBehavior::AbortScript(exit.clone()),
            exit_code_behaviors: vec![ExitCodeRangeBehavior::new(
                ExitCodeRange::Exact(0),
                NEXT_INSTRUCTION.clone(),
            )],
            default_behavior: OnExitBehavior::AbortScript(exit),
        }
    }
}

/// Collects events over a short period of time, after which,
/// when called, will pass out events to listeners and return
/// all those collected events.
pub trait EventCollection {
    /// Gather up the events that happened since the previous call,
    /// pass those to any of its own listeners, then return those events.
    fn collect_and_handle(&mut self) -> Result<Vec<Event>, ScriptExit>;
}

/// In order to work with the ScheduleStep, thread runners utilize a mechanism
/// to collect pending events which it can then use to evaluate the steps,
/// in particular for the WaitForEvent step.
pub trait EventCollector: EventCollection + JobRunnerContext {}

/// A sequence of steps to run in the scheduler.
/// A job thread is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
/// When started, the job thread always starts at the first step.
#[derive(Clone, Debug)]
pub struct ThreadDescription {
    /// The source code or script associated with the job sequence; for debugging.
    pub source: Resource,

    /// The list of steps in the job sequence.
    pub steps: Vec<ScheduleStep>,
}

/// Maintains the read-only store of the script's threads.
///
/// The ThreadStore may be constructed through either the ThreadRegistrar, which
/// gives a programmatic approach to creating jobs with correctly generated
/// ThreadRef IDs for handling with other constructs, or through an explicit
/// creation that requires the caller to keep track of ThreadRef IDs as the
/// index in the passed-in list.
#[derive(Clone, Debug)]
pub struct ThreadStore {
    store: Vec<ThreadDescription>,
}

impl ThreadStore {
    /// Create an explicit job store.  Only use this if the caller keeps
    /// track of ThreadRef == index in the vector; otherwise, build the ThreadStore
    /// through the ThreadRegistrar.
    pub fn new_explicit(threads: Vec<ThreadDescription>) -> Self {
        #[cfg(test)]
        assert!(threads.len() <= MAX_STEPS);
        Self { store: threads }
    }
    /// Get the thread by its reference.
    pub fn get(&self, t_ref: ThreadRef) -> Option<&ThreadDescription> {
        self.store.get(t_ref)
    }

    /// Extract all the descriptions + the reference.
    /// For use with the scheduler that turns the threads into an internal execution state.
    pub fn drain(mut self) -> Vec<(ThreadRef, ThreadDescription)> {
        self.store.drain(0..self.store.len()).enumerate().collect()
    }
}

/// Constructs the set of threads incrementally during initialization stage.
///
/// The thread registration, like all setup stage structures, does not allow for asynchronous construction.
#[derive(Debug)]
pub struct ThreadRegistrar {
    store: Vec<Rc<RefCell<ThreadBuilderInner>>>,
}

/// Allows for adding new threads to the registration.
impl ThreadRegistrar {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    /// Register a new thread for construction.
    pub fn register_from(&mut self, source: Resource) -> ThreadRegistration {
        let t_ref = self.store.len();
        let inner = ThreadBuilderInner::new(t_ref, source);
        self.store.push(inner.clone());
        ThreadRegistration { inner }
    }

    /// Short-hand for constructing the thread registration with the source information.
    pub fn register(
        &mut self,
        name: &str,
        kind: &str,
        file: &str,
        line: i64,
        column: i64,
    ) -> ThreadRegistration {
        self.register_from(Resource::new(name, kind, file, line, column))
    }

    /// Close off thread registration and create the read-only store.
    pub fn close(self) -> Result<ThreadStore, String> {
        let mut store = Vec::new();
        for t in self.store {
            let (t_ref, t) = t.borrow_mut().build()?;
            if store.len() != t_ref {
                return Err(format!(
                    "BUG set wrong thread reference ({}, expected {}) for {}",
                    t_ref,
                    store.len(),
                    t.source,
                ));
            }
            store.push(t);
        }
        Ok(ThreadStore { store })
    }
}

pub struct ThreadRegistration {
    inner: Rc<RefCell<ThreadBuilderInner>>,
}

impl ThreadRegistration {
    /// Get the thread's reference identifier.
    pub fn thread_ref(&self) -> ThreadRef {
        self.inner.borrow().t_ref
    }

    /// Set the thread's steps.
    pub fn set_steps(&mut self, mut steps: Vec<ScheduleStep>) {
        let mut inner = self.inner.borrow_mut();
        inner.steps.clear();
        inner.steps.append(&mut steps);
    }

    /// Set the thread's steps.
    pub fn with_steps(mut self, steps: Vec<ScheduleStep>) -> Self {
        self.set_steps(steps);
        self
    }

    /// Add a step at the end of the thread's current step list.
    pub fn push_step(&mut self, step: ScheduleStep) {
        let mut inner = self.inner.borrow_mut();
        inner.steps.push(step);
    }

    pub fn with_next_step(mut self, step: ScheduleStep) -> Self {
        self.push_step(step);
        self
    }
}

#[derive(Debug)]
struct ThreadBuilderInner {
    t_ref: ThreadRef,
    source: Resource,
    steps: Vec<ScheduleStep>,
}

impl ThreadBuilderInner {
    pub fn new(t_ref: ThreadRef, source: Resource) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            t_ref,
            source,
            steps: Vec::new(),
        }))
    }

    // ... should be a `close(mut self)`, but this is usually wrapped in an Rc.
    pub fn build(&self) -> Result<(ThreadRef, ThreadDescription), String> {
        if self.steps.is_empty() {
            Err(format!("thread {} has no steps", self.source))
        } else if self.steps.len() > MAX_STEPS {
            Err(format!(
                "thread {} has too many steps ({}, limit {})",
                self.source,
                self.steps.len(),
                MAX_STEPS
            ))
        } else {
            Ok((
                self.t_ref,
                ThreadDescription {
                    source: self.source.clone(),
                    steps: self.steps.clone(),
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::event::*;
    use super::super::job::*;
    use super::*;

    #[test]
    fn test_std_build() {
        let (e0, e1) = {
            let mut reg = EventRegistrar::new();
            let e0 = reg.add_event("abort", &EventKind::Signal);
            let e1 = reg.add_event("sig", &EventKind::Signal);
            reg.close();
            (e0, e1)
        };
        let (j0_ref, j1_ref) = {
            let mut reg = JobRegistrar::new();
            let j0 = reg.register(
                JobBuilder::new("j0", "job", "s.sh", 1, 1)
                    .with_listens_to(e0)
                    .with_listens_to(e1)
                    .with_runner(Box::new(SampleJob {}))
                    .close()
                    .expect("j0 close failed"),
            );
            let j1 = reg.register(
                JobBuilder::new("j1", "job", "s.sh", 2, 1)
                    .with_runner(Box::new(SampleJob {}))
                    .close()
                    .expect("j1 close failed"),
            );
            reg.close().expect("should have no errors");
            (j0, j1)
        };

        let mut reg = ThreadRegistrar::new();
        let mut t0 = reg.register("t0", "thread", "s.sh", 1, 1);
        let mut t1 = reg.register("t1", "thread", "s.sh", 2, 1);

        t0.push_step(ScheduleStep::SpawnThread(t1.thread_ref()));
        t0.push_step(ScheduleStep::WaitForThread(
            t1.thread_ref(),
            ExitBehavior::new_ignore_next(),
        ));

        t1.push_step(ScheduleStep::SpawnJob(j0_ref));
        t1.push_step(ScheduleStep::SpawnJob(j1_ref));
        t1.push_step(ScheduleStep::WaitForJob(
            j1_ref,
            ExitBehavior::new_ignore_next(),
        ));

        t0.push_step(ScheduleStep::WaitForJob(
            j0_ref,
            ExitBehavior::new_ignore_next(),
        ));

        reg.close().expect("close failed");
    }

    #[test]
    fn test_invalid_build_none() {
        let mut reg = ThreadRegistrar::new();
        reg.register("t0", "thread", "s.sh", 1, 1);
        assert_eq!(
            "".to_string(),
            reg.close().expect_err("closing did not create an error")
        );
    }

    #[test]
    fn test_invalid_build_many() {
        let e0 = {
            let mut reg = EventRegistrar::new();
            let e0 = reg.add_event("abort", &EventKind::Signal);
            reg.close();
            e0
        };

        let mut reg = ThreadRegistrar::new();
        let mut t0 = reg.register("t0", "thread", "s.sh", 1, 1);
        for _ in 0..(MAX_STEPS + 1) {
            t0.push_step(ScheduleStep::SendEvent(e0, EventPayload::Signal(0)));
        }
        assert_eq!(
            "".to_string(),
            reg.close().expect_err("closing did not create an error")
        );
    }

    struct SampleJob {}
    impl JobRunner for SampleJob {
        fn run(&self, _: Box<dyn JobRunnerContext>) -> Result<ScriptExit, ScriptExit> {
            panic!("not runnable");
        }
    }
}
