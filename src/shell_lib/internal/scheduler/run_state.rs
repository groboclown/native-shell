//! Maintain node state as it executes.

use crate::shell_lib::compile::job;

pub enum JobRunState {
    /// The job has not yet started running.
    NeverStarted,

    /// The node is currently running.
    Running,

    /// The node has stopped execution.
    Stopped,

    /// The node has been aborted.
    Aborted,
}

/// Information about the job's running state.
pub struct JobState {
    pub id: job::JobRef,

    pub state: JobRunState,

    /// The most recent exit code of the node.
    /// If the job was aborted, this will reflect the aborted exit code.
    pub exit_code: Option<job::JobExitCode>,
}

pub enum JobSequenceRunState {
    /// The job sequence has not yet started running.
    NeverStarted,

    /// The job sequence is currently running.
    Running,

    /// The job sequence has stopped execution.
    Stopped,
}

pub struct JobSequenceState {
    /// The index of the action list.
    sequence_ref: job::JobSequenceRef,

    pub state: JobSequenceRunState,

    /// The most recent exit code of the node.
    pub exit_code: Option<job::JobExitCode>,

    active_action_index: u32,
}
