//SPDX:MIT

//! Context arguments for the modules' various actions.

use std::sync::Arc;
use std::sync::mpsc::Sender;

use super::event::{Event, EventKind, EventPayload, EventRef};
use super::job::{JobRunnerContext, ScriptExit};

/// Passed during initialization.
/// Module 'new' functions will look like:
///   pub fn new(
///       source: crate::shell_lib::structure::source::Source,
///       ctx: &mut Box<dyn InitCtx>
///   ) -> Self
pub trait InitCtx {
    /// Get or add the event ID for the event name.
    fn get_event(&mut self, name: &str, kind: &EventKind) -> EventRef;

    /// Send events with the given ID to the channel.
    /// Passing the 'kind' helps enforce that the event types align.
    fn channel_on(&mut self, e_ref: EventRef, kind: &EventKind, ch: Arc<Sender<Event>>);

    /// Send events with the given name to the channel.
    /// Passing the 'kind' helps enforce that the event types align.
    fn channel_on_name(&mut self, event: &str, kind: &EventKind, ch: Arc<Sender<Event>>);

    /// Send events with the given ID to the callback.
    /// Passing the 'kind' helps enforce that the event types align.
    fn callback_on(&mut self, e_ref: EventRef, kind: &EventKind, cb: Arc<Box<dyn EventCallback>>);

    /// Send events with the given name to the callback.
    /// Passing the 'kind' helps enforce that the event types align.
    fn callback_on_name(&mut self, event: &str, kind: &EventKind, cb: Arc<Box<dyn EventCallback>>);
}

/// A callback handler for events.
pub trait EventCallback {
    fn on<'a, 'b, 'c>(
        &'a self,
        context: &'b Box<dyn JobRunnerContext>,
        event_ref: EventRef,
        payload: &'c EventPayload,
    ) -> Result<(), ScriptExit>;
}

/// Passed during execution.
pub trait ExecCtx: JobRunnerContext {}

pub struct JobRunnerCtx(Box<dyn JobRunnerContext>);

impl JobRunnerCtx {
    pub fn new(ctx: Box<dyn JobRunnerContext>) -> Self {
        Self(ctx)
    }
}

impl JobRunnerContext for JobRunnerCtx {
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit> {
        self.0.send_event(event_ref, payload)
    }
}

impl ExecCtx for JobRunnerCtx {}
