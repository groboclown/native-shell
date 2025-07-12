//! Type declaration for use by modules when interacting with the action event bus.


/// Callback type for sending actions to the event bus.  Passed to modules' `exec()`.
pub type SendAction = fn(&str, String) -> ();

pub const ERROR_LOG: &str = "error_log";

