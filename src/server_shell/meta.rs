//SPDX:MIT

//! Definitions for macros and other parts referenced in a script, used by the builder.

use crate::shell_lib::structure::meta::ModuleMeta;

/// The definition for a macro job.
/// Macro jobs act as meta-meta modules - they take the script input and, from that,
/// generate both the library code and the ModuleMeta structure.
pub trait MacroMeta {
    /// Returns whether this macro supports constructing jobs.
    /// If this returns false, then invocations to `build_job` will fail.
    fn supports_jobs(&self) -> bool;

    /// Returns whether this macro supports constructing commands.
    /// If this returns false, then invocations to `build_command` will fail.
    fn supports_commands(&self) -> bool;

    /// Turn the input into a meta job definition.
    /// The job must send results to the writer as a Rust source file.
    fn build_job(
        &self,
        script: &super::lls::model::Metadata,
        mod_name: Vec<String>,
        input: serde_json::Value,
        out: &dyn std::io::Write,
    ) -> Result<MacroModule, String>;

    /// Turn the input into a meta command definition.
    /// The job must send results to the writer as a Rust source file.
    fn build_command(
        &self,
        script: &super::lls::model::Metadata,
        mod_name: Vec<String>,
        input: serde_json::Value,
        writer: &dyn std::io::Write,
    ) -> Result<MacroModule, String>;
}

/// The constructed module from a macro.
pub struct MacroModule {
    pub meta: ModuleMeta,
}
