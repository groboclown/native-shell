//! Type declaration for use by modules when interacting with the action event bus.

use crate::shell_lib::compile::{job, source::Source};

pub const ABORT_EVENT: &str = "abort";
pub const TRACE_LOG: &str = "trace_log";
pub const DEBUG_LOG: &str = "debug_log";
pub const INFO_LOG: &str = "info_log";
pub const WARNING_LOG: &str = "warning_log";
pub const ERROR_LOG: &str = "error_log";


pub fn new_abort_event_group() -> job::EventGroup {
    job::EventGroup {
        name: ABORT_EVENT.to_string(),
    }
}

pub fn new_trace_event_group() -> job::EventGroup {
    job::EventGroup {
        name: TRACE_LOG.to_string(),
    }
}

pub fn new_debug_event_group() -> job::EventGroup {
    job::EventGroup {
        name: DEBUG_LOG.to_string(),
    }
}

pub fn new_info_event_group() -> job::EventGroup {
    job::EventGroup {
        name: INFO_LOG.to_string(),
    }
}

pub fn new_warning_event_group() -> job::EventGroup {
    job::EventGroup {
        name: WARNING_LOG.to_string(),
    }
}

pub fn new_error_event_group() -> job::EventGroup {
    job::EventGroup {
        name: ERROR_LOG.to_string(),
    }
}

pub fn with_default_event_groups(defined: Vec<job::EventGroup>) -> Vec<job::EventGroup> {
    // TODO ensure that each event group is unique.
    defined.into_iter().chain(vec![
        new_abort_event_group(),
        new_trace_event_group(),
        new_debug_event_group(),
        new_info_event_group(),
        new_warning_event_group(),
        new_error_event_group(),
    ]).collect()
}

pub fn send_trace_event(
    context: &Box<dyn job::JobRunnerContext>,
    source: &Source,
    message: String,
) -> Result<(), String> {
    context.as_ref().send_event(&TRACE_LOG.to_string(), job::EventPayload::Message(
        format!("{}: {}", source, message),
    ))
}

pub fn listen_trace_event(context: &Box<dyn job::JobSequenceEventRegistrar>, handler: Box<dyn job::MessageEventHandler + Send + Sync>) {
    let _ = context.add_message_event_listener(&TRACE_LOG.to_string(), handler);
}

pub fn send_debug_event(
    context: &Box<dyn job::JobRunnerContext>,
    source: &Source,
    message: String,
) -> Result<(), String> {
    context.as_ref().send_event(&DEBUG_LOG.to_string(), job::EventPayload::Message(
        format!("{}: {}", source, message),
    ))
}

pub fn listen_debug_event(context: &Box<dyn job::JobSequenceEventRegistrar>, handler: Box<dyn job::MessageEventHandler + Send + Sync>) {
    let _ = context.add_message_event_listener(&DEBUG_LOG.to_string(), handler);
}

pub fn send_info_event(
    context: &Box<dyn job::JobRunnerContext>,
    source: &Source,
    message: String,
) -> Result<(), String> {
    context.as_ref().send_event(&INFO_LOG.to_string(), job::EventPayload::Message(
        format!("{}: {}", source, message),
    ))
}

pub fn listen_info_event(context: &Box<dyn job::JobSequenceEventRegistrar>, handler: Box<dyn job::MessageEventHandler + Send + Sync>) {
    let _ = context.add_message_event_listener(&INFO_LOG.to_string(), handler);
}

pub fn send_warning_event(
    context: &Box<dyn job::JobRunnerContext>,
    source: &Source,
    message: String,
) -> Result<(), String> {
    context.as_ref().send_event(&WARNING_LOG.to_string(), job::EventPayload::Message(
        format!("{}: {}", source, message),
    ))
}

pub fn listen_warning_event(context: &Box<dyn job::JobSequenceEventRegistrar>, handler: Box<dyn job::MessageEventHandler + Send + Sync>) {
    let _ = context.add_message_event_listener(&WARNING_LOG.to_string(), handler);
}

pub fn send_error_event(
    context: &Box<dyn job::JobRunnerContext>,
    source: &Source,
    message: String,
) -> Result<(), String> {
    context.as_ref().send_event(&ERROR_LOG.to_string(), job::EventPayload::Message(
        format!("{}: {}", source, message),
    ))
}

pub fn listen_error_event(context: &Box<dyn job::JobSequenceEventRegistrar>, handler: Box<dyn job::MessageEventHandler + Send + Sync>) {
    let _ = context.add_message_event_listener(&ERROR_LOG.to_string(), handler);
}
