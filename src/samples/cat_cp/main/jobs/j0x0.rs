//! Generated from job j0x0.
//! This comes from the stream job builder.

use std::sync::Arc;

use super::super::modules;
use super::runtime;
use crate::shell_lib::{stream, structure};

pub struct Jobj0x0 {
    state: Arc<modules::j0x0::Jobj0x0State>,
}

impl Jobj0x0 {
    pub fn new(ctx: &mut dyn structure::InitCtx) -> Result<Self, structure::ScriptExit> {
        // Create the state.
        let state = Arc::new(modules::j0x0::Jobj0x0State::new());

        // Register the abort handler.
        ctx.callback_on_name(
            "abort",
            &structure::EventKind::Signal,
            Arc::new(Box::new(Jobj0x0Abort {
                state: state.clone(),
            }) as Box<dyn structure::EventCallback>),
        );

        Ok(Self { state })
    }

    pub fn runner(&self, runtime: runtime::Runtime) -> structure::job::JobDescription {
        structure::job::JobDescription {
            source: structure::source::Resource {
                name: "j0x0".into(),
                kind: "stream-job".into(),
                source: structure::Source::new("script.ns", 0, 0),
            },
            rerunable: true,
            runner: Box::new(Jobj0x0Runner {
                state: self.state.clone(),
                runtime,
            }),
        }
    }

    pub fn state(&self) -> modules::j0x0::Jobj0x0State {
        self.state.as_ref().clone()
    }
}

struct Jobj0x0Runner {
    state: Arc<modules::j0x0::Jobj0x0State>,
    runtime: runtime::Runtime,
}

impl structure::job::JobRunner for Jobj0x0Runner {
    fn run(
        &self,
        _ctx: Box<dyn structure::job::JobRunnerContext>,
    ) -> Result<structure::ScriptExit, structure::ScriptExit> {
        // Create the streams.

        // This uses the command state, so it needs to pull in the command state first.
        // This allows for borrowing references rather than performing more data copies.
        let command_state = self.runtime.command_state();

        // stream0
        //   to: filename: lookup from default.target
        let stream0_filename = match &command_state {
            runtime::CommandState::Cdefault(c) => &c.args.target,
        };
        let stream0_file = std::fs::File::open(stream0_filename)?;
        let (stream0, stream0_out) = stream::fd::FdOut::from_file(stream0_file).into_fd();

        // Load up the state.
        self.state.open(stream0_out, stream0)?;

        Ok(0.into())
    }
}

struct Jobj0x0Abort {
    state: Arc<modules::j0x0::Jobj0x0State>,
}

impl structure::EventCallback for Jobj0x0Abort {
    fn on<'a, 'b, 'c>(
        &'a self,
        _context: &'b Box<dyn structure::job::JobRunnerContext>,
        _event_ref: structure::EventRef,
        _payload: &'c structure::EventPayload,
    ) -> Result<(), structure::ScriptExit> {
        self.state.close()
    }
}
