//SPDX:MIT

use std::sync::Arc;

use crate::server_shell::lls::model;

use super::super::collect;
use super::super::errors;
use super::super::writer;

/// Write the rust files related to jobs.
/// Returns an empty result for quick returns.
pub fn gen_jobs(
    issues: errors::ScriptIssues,
    collector: collect::Collector,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    todo!()
}
