//! Turns the AST into a module source.

use std::sync::Arc;

use super::{errors, writer};
use crate::server_shell::lls::model;

const LLS_VERSION_1: &str = "1.0.0";

enum LlsVersion {
    V1,
    Unknown(errors::BuilderError),
}

pub fn lls_to_module_source(
    lls: &model::NativeShellLowLevelScriptSchema,
    out: Arc<dyn writer::SourceWriter + Send + Sync>,
) -> errors::ScriptIssues {
    match get_script_version(lls) {
        LlsVersion::V1 => super::lls_v1::lls_v1_to_module_source(lls, out),
        LlsVersion::Unknown(e) => {
            let issues = errors::ScriptIssues::new();
            issues.add_err(e);
            issues
        }
    }
}

pub fn get_script_version(lls: &model::NativeShellLowLevelScriptSchema) -> LlsVersion {
    if lls.schema_version == LLS_VERSION_1 {
        LlsVersion::V1
    } else {
        LlsVersion::Unknown(errors::BuilderError::InvalidLLS(errors::ErrorDetails {
            message: format!(
                "Unsupported LLS version: {}. Supported versions include: {}",
                lls.schema_version, LLS_VERSION_1
            ),
            source: (&lls.meta.source).into(),
            related: vec![],
        }))
    }
}
