//! Defines the job that the compiler constructs to allow for interaction with the scheduler.
//! These loosely map to the actions and execution of the modules in the script.

use std::fmt::{Display, Write};

pub type ExitCode = i32;
pub type JobRef = usize;
pub type JobSequenceRef = usize;

/// Events are sent as names from the modules.
pub type EventRef = String;

/// A single step in a scheduled sequence.
#[derive(Clone, Debug)]
pub enum ScheduleStep {
    /// Send an event to the event group.
    SendEvent(EventRef, EventPayload),

    /// Spawn (or rerun) the job in the scheduler.  Does nothing if the job is already running.
    /// The job has a single run state, but it may be invoked through different job references.
    /// A single job may also be active in several sequences.
    /// This does not wait for the job to finish.
    SpawnJob(JobRef),

    /// Wait for a job to finish executing.  This will block until the job's execution exits.
    /// The first parameter is the job reference.
    /// The second is how to handle the job's exit code, or if it has never started.
    WaitForJob(JobRef, ExitCodeBehavior),

    /// Spawn a job sequence.  This will run the sequence in parallel.
    SpawnJobSequence(JobSequenceRef),

    /// Wait for a job sequence to finish executing.  This will block until the sequence's execution exits.
    WaitForJobSequence(JobSequenceRef, ExitCodeBehavior),

    /// Stop the whole script execution with the given message.
    Abort(String),
}

/// How to handle the job sequence execution after a step completes.
#[derive(Clone, Debug)]
pub enum OnExitBehavior {
    /// Run the next action in the sequence.
    RunNext,

    /// Skip the next action in the sequence.
    SkipNext,

    /// Skip all remaining actions in the sequence.
    /// The final exit code will be the previous job's exit code, or `ExitCode::MIN` if the job has never started.
    SkipAll,

    /// Abort the script execution.
    AbortScript,
}

/// Covers a range of exit codes.
#[derive(Clone, Debug)]
pub enum ExitCodeRange {
    /// A single exit code.
    Exact(ExitCode),

    /// A range of exit codes, inclusive.
    Range(ExitCode, ExitCode),

    AtOrAbove(ExitCode),

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
            (None, None) => panic!("Cannot create an exit code range with no bounds; use default in behavior instead"),
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
    /// Check if the given exit code is within the range and return the behavior.
    pub fn matches(&self, code: ExitCode) -> Option<OnExitBehavior> {
        if self.code.contains(code) {
            Some(self.behavior.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExitCodeBehavior {
    /// The default behavior if the job has never started.
    pub never_started: OnExitBehavior,

    /// The behaviors based on the exit code of the job.
    pub exit_code_behaviors: Vec<ExitCodeRangeBehavior>,

    pub default_behavior: OnExitBehavior,
}

impl ExitCodeBehavior {
    /// Check if the given exit code matches any of the behaviors.
    pub fn behavior_for(&self, code: ExitCode) -> OnExitBehavior {
        for behavior in &self.exit_code_behaviors {
            if let Some(b) = behavior.matches(code) {
                return b;
            }
        }

        self.default_behavior.clone()
    }
}

/// A sequence of steps to run in the scheduler.
/// A job sequence is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
#[derive(Clone, Debug)]
pub struct JobSequenceDescription {
    /// The name of the job sequence; for debugging.
    pub name: String,

    /// The source code or script associated with the job sequence; for debugging.
    pub source: String,

    /// The list of steps in the job sequence.
    pub steps: Vec<ScheduleStep>,
}

pub trait JobRunner {
    fn run(&self, context: Box<dyn JobRunnerContext>) -> Result<ExitCode, String>;
    fn abort(&self) -> Result<(), String>;
}

/// An event handler specializing in string messages.
/// 
/// These generally come from the compiler creating special handlers, such as for logging.
pub trait MessageEventHandler {
    fn handle_message<'a>(&self, message: String, scheduler: &Box<dyn EventHandlerContext + 'a>) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct SignalRangeBehavior {
    /// The signal range covered by this behavior.
    pub code: ExitCodeRange,

    /// The behavior to take based on the signal.
    pub behavior: JobSequenceRef,
}

impl SignalRangeBehavior {
    /// Check if the given exit code is within the range and return the behavior.
    pub fn matches(&self, code: ExitCode) -> Option<JobSequenceRef> {
        if self.code.contains(code) {
            Some(self.behavior.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct SignalEventHandler {
    /// The behaviors based on the signal.
    pub signal_behaviors: Vec<SignalRangeBehavior>,

    pub default_behavior: JobSequenceRef,
}

impl SignalEventHandler {
    /// Check if the given exit code matches any of the behaviors.
    pub fn behavior_for(&self, code: ExitCode) -> JobSequenceRef {
        for behavior in &self.signal_behaviors {
            if let Some(b) = behavior.matches(code) {
                return b;
            }
        }

        self.default_behavior.clone()
    }
}

/// Allows a running job to handle events broadcast through the scheduler.
pub enum EventHandler {
    Message(Box<dyn MessageEventHandler + Send + Sync>),
    Signal(SignalEventHandler),
}

#[derive(Clone, Debug)]
pub struct EventGroup {
    /// The name of the event group; for debugging.
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum EventPayload {
    /// An event with a string payload.
    Message(String),

    /// An event with an integer payload.
    Signal(ExitCode),
}

impl Display for EventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => {
                f.write_str("EventPayload::Message(")?;
                f.write_str(m)?;
                f.write_str(")")
            }
            Self::Signal(s) => {
                f.write_str("EventPayload::Signal(")?;
                f.write_str(&s.to_string())?;
                f.write_str(")")
            }
        }
    }
}

/// Context sent to the job runner to allow it to have limited interaction with the scheduler.
/// Note that the job itself should not manage job state (start and wait), as that's handled by the job sequences.
/// If a job needs to handle things like conditions, then use the exit code behavior.
pub trait JobRunnerContext {
    /// Send an event to the event group.
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), String>;
}

pub type EventGroupListen = std::collections::HashMap<EventRef, EventHandler>;

/// A description of a job that is used to run behavior in the scheduler.
/// These are created by the script compiler and executed by the engine.
/// A job is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
pub struct JobDescription {
    /// The name of the job; for debugging.
    pub name: String,

    /// The source code or script associated with the job; for debugging.
    pub source: String,

    /// The job will listen to these event groups, with the associated sequence, while it runs.
    /// TODO: need to figure out if this should attach to the job or the sequence.
    pub listen: Option<EventGroupListen>,

    /// Job handler.
    pub runner: Box<dyn JobRunner + Sync + Send>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobSequenceRunState {
    /// The job sequence has not yet started running.
    NeverStarted,

    /// The job sequence is currently running.
    /// Note that, if the job sequence has run before, its exit code is not available,
    /// because it can only be accessed at stop time through the wait request.
    Running,

    /// The job sequence has finished running.
    Finished(ExitCode),
}

/// High level scheduler interface for job management.
pub trait JobScheduler {
    /// Start a job sequence.
    /// Does nothing if the job is actively running.
    fn start_job_sequence(&self, seq_ref: JobSequenceRef) -> Result<(), String>;

    /// Get the current state of the job sequence.
    fn get_job_sequence_state(&self, seq_ref: JobSequenceRef) -> JobSequenceRunState;

    /// Wait for a job sequence to finish executing.
    /// Returns immediately if the sequence has not started.
    fn wait_for_job_sequence(&self, seq_ref: JobSequenceRef, timeout: Option<std::time::Duration>) -> Result<Option<ExitCode>, String>;

    /// Send a signal to stop all jobs in progress.
    fn abort(&self);
}


pub trait EventHandlerContext: JobScheduler + JobRunnerContext {}
