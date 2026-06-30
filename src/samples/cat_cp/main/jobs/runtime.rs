use std::sync::Arc;

use super::default;
use super::j0x0;
use super::j0x1;

/// Collection of all jobs.
/// This allows referencing other jobs at execution time.
pub struct Jobs {
    pub default: default::Jobdefault,
    pub j0x0: j0x0::Jobj0x0,
    pub j0x1: j0x1::Jobj0x1,
}

#[derive(Clone)]
pub struct Runtime {
    pub jobs: Arc<Jobs>,
}
