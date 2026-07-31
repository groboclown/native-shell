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

pub fn gen_inline_job_file(
    settings: super::jobs::JobSettings,
    job: Arc<lls::model::InlineJob>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let mut out = issues.consume(out.writer_for(&settings.job_file))?;
    helpers::write_string(
        &mut out,
        &issues,
        helpers::rust_file_header(&collector.meta, &settings.now),
    )?;

    todo!()
}
