//! Traits that nodes need to implement to work with the shell engine.
//! Nodes are built from the script.  A node has an Arc reference to the other nodes it
//! depends on for fetching state.  It also has a set of action lists which the execution
//! engine references by index.

use super::action::ActionRef;

pub type NodeRef = u32;

/// A module instance that can be executed by the engine.
/// These are created by the script compiler and executed by the engine.
pub trait Node {
    /// The node has a list of actions it performs.  The code uses an index for each action list.
    fn run_action(&self, action_ref: ActionRef) -> Result<(), String>;

    fn stop(&self, context: Box<dyn EngineContext>) -> Result<(), String>;

    fn execute(&self, context: Box<dyn EngineContext>) -> Result<i16, String>;
}

/// Context sent to a node to allow it to have limited interaction with the engine.
pub trait EngineContext {
    /// Send an event to the event group.
    fn send_event(&self, group: String, message: Option<String>, code: Option<i64>) -> Result<(), String>;

    /// Start or restart a node.  Performs a no-op for running nodes.
    /// Does not wait for the node to finish.
    fn start_node(&self, node_ref: NodeRef) -> Result<(), String>;

    /// Stop the script execution with the given message.
    fn abort_script(&self, message: String) -> Result<(), String>;

    /// Run an action on the current node.
    fn run_node_action(&self, action_ref: ActionRef) -> Result<(), String>;
}
