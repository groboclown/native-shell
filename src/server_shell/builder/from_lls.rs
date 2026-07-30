//! Turns the LLS into a module source.

use std::sync::Arc;

use super::{errors, writer};
use crate::server_shell::lls::model;

const LLS_VERSION_1: &str = "1.0.0";

pub enum LlsVersion {
    V1,
    Unknown(errors::BuilderError),
}

pub fn lls_to_module_source(
    script: &model::NativeShellLowLevelScriptSchema,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> errors::ScriptIssues {
    match get_script_version(script) {
        LlsVersion::V1 => super::lls_v1::lls_v1_to_module_source(script, out),
        LlsVersion::Unknown(e) => {
            let issues = errors::ScriptIssues::new();
            issues.add_err(e);
            issues
        }
    }
}

pub fn get_script_version(script: &model::NativeShellLowLevelScriptSchema) -> LlsVersion {
    if script.schema_version == LLS_VERSION_1 {
        LlsVersion::V1
    } else {
        LlsVersion::Unknown(errors::BuilderError::InvalidLLS(errors::ErrorDetails {
            message: format!(
                "Unsupported LLS version: {}. Supported versions include: {}",
                script.schema_version, LLS_VERSION_1
            ),
            source: (&script.meta.source).into(),
            related: vec![],
        }))
    }
}
