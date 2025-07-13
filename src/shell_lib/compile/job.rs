//! Defines the job that the compiler constructs to allow for interaction with the scheduler.
//! These loosely map to the actions and execution of the modules in the script.

use super::event::EventRef;

pub type JobRef = u32;
pub type JobSequenceRef = u32;

pub type JobExitCode = i16;

/// A single step in a scheduled sequence.
pub enum ScheduleStep {
    /// Send an event to the event group.
    SendEvent((String, Option<String>, Option<i64>)),

    /// Spawn (or rerun) the job in the scheduler.  Does nothing if the job is already running.
    /// The job has a single run state, but it may be invoked through different job references.
    /// A single job may also be active in several sequences.
    /// This does not wait for the job to finish.
    SpawnJob(JobRef),

    /// Wait for a job to finish executing.  This will block until the job's execution exits.
    /// The first parameter is the job reference.
    /// The second is how to handle the case where the node has never started.
    /// The third is the behavior to take based on the exit code of the node.
    WaitForJob(JobRef, OnExitBehavior, ExitCodeBehavior),

    /// Spawn a job sequence.  This will run the sequence in parallel.
    SpawnJobSequence(JobSequenceRef),

    /// Wait for a job sequence to finish executing.  This will block until the sequence's execution exits.
    WaitForJobSequence(JobSequenceRef, OnExitBehavior, ExitCodeBehavior),

    /// Stop the whole script execution with the given message.
    Abort(String),
}

/// How to handle the job sequence execution after a step completes.
pub enum OnExitBehavior {
    /// Run the next action in the sequence.
    RunNext,

    /// Skip the next action in the sequence.
    SkipNext,

    /// Skip all remaining actions in the sequence.
    SkipAll,

    /// Abort the script execution.
    AbortScript,
}

/// Possible ranges of job exit codes.
pub enum ExitCodeRange {
    /// The exit code is not specified.
    Any,

    /// The exit code is in the range [min, max].
    Range { min: JobExitCode, max: JobExitCode },

    /// The exit code is equal or above the given value.
    AtOrAbove(JobExitCode),

    /// The exit code is equal or below the given value.
    AtOrBelow(JobExitCode),

    /// The exit code is exactly the given value.
    Exact(JobExitCode),
}

/// How to handle the job sequence execution after a job completes.
pub struct ExitCodeBehavior {
    /// The exit code range covered by this behavior.
    pub code: ExitCodeRange,

    /// The behavior to take based on the exit code of the node.
    pub behavior: OnExitBehavior,
}

/// A sequence of steps to run in the scheduler.
/// A job sequence is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
pub struct JobSequence {
    /// The unique identifier for the job sequence.
    pub id: JobSequenceRef,

    /// The name of the job sequence; for debugging.
    pub name: String,

    /// The source code or script associated with the job sequence; for debugging.
    pub source: String,

    /// The list of steps in the job sequence.
    pub steps: Vec<ScheduleStep>,
}

type JobRunner = Box<dyn Fn(Box<dyn SchedulerContext>) -> Result<JobExitCode, String> + Send + Sync>;

pub type EventGroupListen = std::collections::HashMap<EventRef, JobSequenceRef>;

/// A description of a job that is used to run a node in the scheduler.
/// These are created by the script compiler and executed by the engine.
/// A job is a singleton; it may be restarted or stopped, but it cannot be
/// run multiple times in parallel.
pub struct JobDescription {
    /// The unique identifier for the job.
    pub id: JobRef,

    /// The name of the job; for debugging.
    pub name: String,

    /// The source code or script associated with the job; for debugging.
    pub source: String,

    /// The job will listen to these event groups, with the associated sequence, while it runs.
    pub listen: Option<EventGroupListen>,

    /// Run the job.
    pub runner: JobRunner,

    /// Abort the job if it is running and if `abort_script` is called.
    pub abort_handler: Option<JobRunner>,
}

/// Context sent to the job runner to allow it to have limited interaction with the scheduler.
/// Note that the job itself should not manage job state (start and wait), as that's handled by the job sequences.
/// If a job needs to handle things like conditions, then use the exit code behavior.
pub trait SchedulerContext {
    /// Send an event to the event group.
    fn send_event(&self, group: EventRef, message: Option<String>, code: Option<i64>) -> Result<(), String>;
}
