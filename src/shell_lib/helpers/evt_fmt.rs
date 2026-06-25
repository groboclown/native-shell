//SPDX:MIT

use std::fmt::Arguments;

use crate::shell_lib::structure::{
    event::{EventPayload, EventRef},
    job::{JobRunnerContext, ScriptExit},
    source::Source,
};

pub fn fmt_log(source: &Source, msg: Arguments) -> EventPayload {
    EventPayload::Message(format!("({}) {}", source, msg))
}

pub fn send_log(
    context: &Box<dyn JobRunnerContext>,
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
