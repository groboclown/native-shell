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

pub fn gen_macro_job_file(
    settings: super::jobs::JobSettings,
    job: Arc<lls::model::MacroJob>,
    mac: Arc<Box<dyn meta::MacroMeta + Send + Sync>>,
    collector: collect::Collector,
    issues: errors::ScriptIssues,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> Result<(), ()> {
    let mut out = issues.consume(
        out.writer_for(
            format!(
                "{}/{}.rs",
                settings.parent_module.join("/"),
                settings.module_name
            )
            .as_str(),
        )
        .map_err(|e| e.into()),
    )?;
    helpers::write_string(
        &mut out,
        &issues,
        helpers::rust_file_header(&collector.meta, &settings.now),
    )?;

    todo!()
}
