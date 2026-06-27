//SPDX:MIT

//! Common logging tools.
//! While the shell library doesn't have an explicit logging mechanism, it instead uses
//! events to pass logs around for proper handling.  This means, though, that the
//! log system requires proper registration.

use std::fmt::Arguments;

use crate::shell_lib::structure::{
    EventKind, EventPayload, EventRef, ExecCtx, InitCtx, ScriptExit, Source,
};

pub const TRACE: &str = "trace";
pub const DEBUG: &str = "debug";
pub const VERBOSE: &str = "verbose";
pub const INFO: &str = "info";
pub const NOTICE: &str = "notice";
pub const WARNING: &str = "warning";
pub const ERROR: &str = "error";

/// A structure that a module can use for handling logging requests.
pub struct Logger {
    source: Source,
    trace: EventRef,
    debug: EventRef,
    verbose: EventRef,
    info: EventRef,
    notice: EventRef,
    warning: EventRef,
    error: EventRef,
}

impl Logger {
    /// Create a new logger that's capable of sending out log messages easily.
    pub fn new(source: &Source, ctx: &mut dyn InitCtx) -> Self {
        Self {
            source: source.clone(),
            trace: ctx.get_event(TRACE, &EventKind::Message),
            debug: ctx.get_event(DEBUG, &EventKind::Message),
            verbose: ctx.get_event(VERBOSE, &EventKind::Message),
            info: ctx.get_event(INFO, &EventKind::Message),
            notice: ctx.get_event(NOTICE, &EventKind::Message),
            warning: ctx.get_event(WARNING, &EventKind::Message),
            error: ctx.get_event(ERROR, &EventKind::Message),
        }
    }

    /// Send a trace message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn trace(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.trace, fmt_log(&self.source, msg))
    }

    /// Send a debug message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn debug(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.debug, fmt_log(&self.source, msg))
    }

    /// Send a verbose message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn verbose(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.verbose, fmt_log(&self.source, msg))
    }

    /// Send an informative message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn info(&self, context: &Box<dyn ExecCtx>, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.info, fmt_log(&self.source, msg))
    }

    /// Send a notice message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn notice(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.notice, fmt_log(&self.source, msg))
    }

    /// Send a warning message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn warning(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.warning, fmt_log(&self.source, msg))
    }

    /// Send an error message.
    /// Use `format_args!()` macro to construct the message parameter.
    pub fn error(&self, context: &mut dyn ExecCtx, msg: Arguments) -> Result<(), ScriptExit> {
        context.send_event(self.error, fmt_log(&self.source, msg))
    }
}

/// Construct a payload for a log message.
pub fn fmt_log(source: &Source, msg: Arguments) -> EventPayload {
    EventPayload::Message(format!("({}) {}", source, msg))
}

/// Send a log message.
pub fn send_log(
    context: &Box<dyn ExecCtx>,
    e_ref: EventRef,
    source: &Source,
    msg: Arguments,
) -> Result<(), String> {
    context
        .send_event(e_ref, fmt_log(source, msg))
        .map_err(|e| match e.message {
            Some(m) => m,
            None => format!("{}", e.code),
        })
}
