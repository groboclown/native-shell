
pub type ActionRef = u32;


/// A single action step in a module's action sequence.
pub enum ActionStep {
    /// Send an event to the event group.
    SendEvent((String, Option<String>, Option<i64>)),

    /// Run (or rerun) the named node.  Does nothing if the node is already running.
    RunNode(String, Vec<ExitCodeBehavior>),

    /// Run an action handler associated with the node.
    RunModuleAction(u16),

    /// Stop the script execution with the given message.
    Abort(String),
}

pub struct Action {
    pub index: ActionRef,
    pub step: ActionStep,
}

pub type ActionSequence = Vec<Action>;

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

pub struct ExitCodeBehavior {
    /// The exit code range for this behavior.
    pub min: Option<i16>,
    pub max: Option<i16>,

    /// The behavior to take based on the exit code of the node.
    pub behavior: OnExitBehavior,
}
