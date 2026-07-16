//SPDX:MIT

use std::sync::Arc;

use crate::server_shell::lls;
use crate::server_shell::meta;
use crate::shell_lib::structure;

use super::super::collect;
use super::super::errors;
use super::super::writer;
use super::helpers;
use super::names;

/// Write the rust files related to jobs.
/// Returns an empty result for quick returns.
pub fn gen_jobs(
    issues: errors::ScriptIssues,
    collector: collect::Collector,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    // 1. Each job (non-command) is written independently.

    // 2. Create the job index while the job generators run.

    // 3. Wait for the job generators to complete.

    Ok(())
}

/// Defines various parts of the job, for constructing the file.
pub struct JobSettings {
    pub name: String,
    pub source: lls::model::Source,
    pub job_ref: structure::JobRef,
    pub parent_module: Vec<String>,
    pub module_name: String,
    pub struct_name: String,
    pub init_param_struct: String,
    pub runtime_param_struct: String,
    pub state_struct: String,
    pub now: String,
}
