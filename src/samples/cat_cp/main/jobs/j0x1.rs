//! The cat module as a job.

use std::{os::fd::OwnedFd, sync::Arc};

use super::runtime;
use crate::shell_lib::{modules::cat, structure};

pub struct Jobj0x1 {
    state: Arc<cat::CatModule>,
}

impl Jobj0x1 {
    pub fn new(ctx: &mut dyn structure::InitCtx) -> Result<Self, structure::ScriptExit> {
        let state = Arc::new(cat::CatModule::new(
            structure::Source::new("script.ns", 0, 0),
            ctx,
            // No compile-time parameters.
        ));

        // No handlers declared.

        Ok(Self { state })
    }

    pub fn runner(&self, runtime: runtime::Runtime) -> structure::job::JobDescription {
        structure::job::JobDescription {
            source: structure::source::Resource {
                name: "j0x1".into(),
                kind: "module-job".into(),
                source: structure::Source::new("script.ns", 0, 0),
            },
            rerunable: true,
            runner: Box::new(Jobj0x1Runner {
                state: self.state.clone(),
                runtime,
            }),
        }
    }

    // no state
}

struct Jobj0x1Runner {
    state: Arc<cat::CatModule>,
    runtime: runtime::Runtime,
}

impl structure::job::JobRunner for Jobj0x1Runner {
    fn run(
        &self,
        ctx: Box<dyn structure::job::JobRunnerContext>,
    ) -> Result<structure::ScriptExit, structure::ScriptExit> {
        let mut ctx = structure::JobRunnerCtx::new(ctx);
        let params = cat::CatModuleRuntimeParams {
            filenames: vec![
                // filenames[0] is lookup from default.source
                self.runtime.jobs.default.state().source.clone(),
            ],
        };
        let stream0_out: std::io::Result<OwnedFd> =
            self.runtime.jobs.j0x0.state().take_stream0_out()?.into();
        let streams = cat::CatModuleStream { fd_0: stream0_out? };
        self.state.exec(&mut ctx, params, streams)
    }
}
