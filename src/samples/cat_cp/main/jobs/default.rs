//! The job that comes from the 'default' thread's main parameters + environment variables.
//! As a main parameter "job", it doesn't have a run implementation because it doesn't run.

use std::sync::Arc;

use super::super::modules;
use crate::shell_lib::structure;

pub struct Jobdefault {
    state: Arc<modules::default::JobdefaultParameters>,
}

impl Jobdefault {
    pub fn new(
        _ctx: &mut dyn structure::InitCtx,
        params: modules::default::JobdefaultParameters,
    ) -> Result<Self, structure::ScriptExit> {
        Ok(Self {
            state: Arc::new(params),
        })
    }

    pub fn state(&self) -> &modules::default::JobdefaultParameters {
        &self.state
    }
}
