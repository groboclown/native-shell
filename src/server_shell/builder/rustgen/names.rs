//SPDX:MIT

//! Functions that generate Rust-specific names for elements.
//! This allows for the generators to share this logic to ensure that they
//! refer to the same things.
//!
//! General naming convention:
//!    *_module_name() -> the name of the module, might be a directory name, the '<name>.rs' file, or the
//!                       Rust code module reference.
//!    *_MODULE_NAME   -> a &str constant, if the module name is static.
//!    *_module_path() -> the relative path of the module (Vec<String>).  Can be used to construct the
//!                       file system path or the module path.
//!    *_mod() ->         the absolute module path (Vec<String>); includes 'crate' at the start.
//!    *_file() ->        the OS path to the file (if it can be referenced as a file), relative to the project root.
//!    *_dir() ->         the OS path to the directory (if it can be referenced as a directory), relative to the project root.

use crate::shell_lib::structure::JobRef;

// --------------------------------------------------------------------
// General names.

/// Rust's prefix to use the crate libraries.
pub const CRATE_MODULE_NAME: &'static str = "crate";

/// Rust's prefix to use the crate libraries.
pub fn crate_module_name() -> String {
    CRATE_MODULE_NAME.to_string()
}

/// Location of the root source directory, inside the project directory.
pub const SRC_DIR_NAME: &'static str = "src";

/// Location of the source directory.
pub fn src_dir() -> std::path::PathBuf {
    SRC_DIR_NAME.into()
}

pub const RUST_DOT_EXT: &'static str = ".rs";
pub const RUST_EXT: &'static str = "rs";

// --------------------------------------------------------------------
// Rust Standard Libraries

pub const STD_NONE: &'static str = "std::option::Option::None";

// --------------------------------------------------------------------
// Shell Library Names

pub const SHELL_LIB: &'static str = "crate::shell_lib";
pub const SHELL_LIB_STRUCT: &'static str = "crate::shell_lib::structure";
pub const SHELL_LIB_SOURCE: &'static str = "crate::shell_lib::structure::Source";

// --------------------------------------------------------------------
// Runtime structure
//
// The crate::runtime structure contains references to all the commands and jobs,
// so that they can read each other's state at runtime.

/// Module name of the runtime structures.
pub const RUNTIME_MODULE_NAME: &'static str = "runtime";

/// Module name of the runtime structures.
pub fn runtime_module_name() -> String {
    RUNTIME_MODULE_NAME.to_string()
}

/// Relative path to the runtime module.
pub fn runtime_module_path() -> Vec<String> {
    vec![runtime_module_name()]
}

/// Absolute Rust module path for the runtime module.
pub fn runtime_mod() -> Vec<String> {
    let mut ret = vec![crate_module_name()];
    ret.append(&mut runtime_module_path());
    ret
}

/// The runtime filename.
pub fn runtime_file() -> std::path::PathBuf {
    let mut ret = src_dir();
    ret.push(RUNTIME_MODULE_NAME);
    ret.with_added_extension(RUST_EXT)
}

/// The 'pub struct {}' name.
pub const RUNTIME_STRUCT_NAME: &'static str = "Runtime";

/// The job 'runner' structure contains the 'runtime' field, a copy of the Arc<Runtime>.
pub const RUNNER_RUNTIME_FIELD_NAME: &'static str = "runtime";

// --------------------------------------------------------------------
// The commands module and structure within the runtime module.
//
// Commands have a special use within the runtime action.
// There exists exactly one active command at runtime; either this comes from
// the default command (or if the script defines just one command), or from
// the user selecting the command to run.

/// The 'pub enum {}' structure created in the runtime module that contains all commands.
/// Only created for scripts with multiple commands.
pub const COMMAND_RUNTIME_ENUM_STRUCT_NAME: &'static str = "Commands";

/// The 'pub enum {}' structure returned by the runtime
/// COMMAND_RUNTIME_STRUCT_GETTER_NAME that contains the current command's state.
pub const COMMAND_RUNTIME_STATE_ENUM_STRUCT_NAME: &'static str = "CommandState";

pub fn command_runtime_state_enum_struct_name() -> String {
    COMMAND_RUNTIME_STATE_ENUM_STRUCT_NAME.to_string()
}

/// The function in the Runtime structure that constructs the command wrapper.
/// This will be the command wrapper itself, for scripts with one command, or the
/// runtime enum, for scripts with multiple commands.
/// This function takes no arguments and returns a
/// COMMAND_RUNTIME_STATE_ENUM_STRUCT_NAME value.
pub const COMMAND_RUNTIME_STRUCT_GETTER_NAME: &'static str = "command_state";

/// The name of the module containing the command modules.
pub const COMMANDS_MODULE_NAME: &'static str = "commands";

pub fn commands_module_name() -> String {
    COMMANDS_MODULE_NAME.to_string()
}

pub fn commands_module_path() -> [&'static str; 1] {
    [COMMANDS_MODULE_NAME]
}

pub fn commands_mod() -> [&'static str; 2] {
    [CRATE_MODULE_NAME, COMMANDS_MODULE_NAME]
}

/// The directory containing the command sources.
pub fn commands_dir() -> std::path::PathBuf {
    let mut ret = src_dir();
    for el in commands_module_path() {
        ret.push(el);
    }
    ret
}

/// The module file for the commands.
pub fn commands_file() -> std::path::PathBuf {
    commands_dir().with_added_extension(RUST_EXT)
}

/// Create the command module name.
/// Usable for both the filename creation and the 'use mod' line.
pub fn command_ref_module_name(job: JobRef) -> String {
    format!("c{}", job)
}

/// Relative path to the command module.
pub fn command_ref_module_path(job: JobRef) -> Vec<String> {
    vec![commands_module_name(), command_ref_module_name(job)]
}

/// The full command module name.
pub fn command_ref_mod(job: JobRef) -> Vec<String> {
    let mut ret = vec![crate_module_name()];
    ret.append(&mut command_ref_module_path(job));
    ret
}

/// The location of the Rust source file containing the command.
pub fn command_ref_file(job: JobRef) -> std::path::PathBuf {
    let mut ret = src_dir();
    for el in command_ref_module_path(job) {
        ret.push(el);
    }
    ret.with_added_extension(RUST_EXT)
}

/// Create the name of the command's state holding structure.
pub fn command_mod_struct(job: JobRef) -> String {
    format!("Cmd{}State", job)
}

/// Create the name of the command's CommandSetup + CommandHandler implementing structure.
pub fn command_run_struct(job: JobRef) -> String {
    format!("Cmd{}Runner", job)
}

/// Create the name of the command wrapper structure within the runtime structure.
/// For scripts that contain more than one command, the runtime structure uses an
/// enum to contain the command requested by the end-user.
/// This name is shared between the state enum and runner enum.
pub fn command_runtime_enum_name(job: JobRef) -> String {
    format!("C{}", job)
}

// --------------------------------------------------------------------

/// The name of the module containing the job modules.
pub const JOBS_MODULE_NAME: &'static str = "jobs";

pub fn jobs_module_name() -> String {
    JOBS_MODULE_NAME.to_string()
}

pub fn jobs_module_path() -> [&'static str; 1] {
    [JOBS_MODULE_NAME]
}

pub fn jobs_mod() -> [&'static str; 2] {
    [CRATE_MODULE_NAME, JOBS_MODULE_NAME]
}

/// Directory containing the jobs module files.
pub fn jobs_dir() -> std::path::PathBuf {
    let mut ret = src_dir();
    for el in jobs_module_path() {
        ret.push(el);
    }
    ret
}

/// File wrapping the jobs module.
pub fn jobs_file() -> std::path::PathBuf {
    jobs_dir().with_added_extension(RUST_EXT)
}

/// Create the job module name.
/// Usable for both the filename creation and the 'use mod' line.
pub fn job_ref_module_name(job: JobRef) -> String {
    format!("j{}", job)
}

pub fn job_ref_module_path(job: JobRef) -> Vec<String> {
    let mut ret: Vec<String> = Vec::from_iter(jobs_module_path().iter().map(|v| v.to_string()));
    ret.push(job_ref_module_name(job));
    ret
}

/// The full job module (crate::...).
pub fn job_ref_mod(job: JobRef) -> Vec<String> {
    let mut ret = vec![crate_module_name()];
    ret.append(&mut job_ref_module_path(job));
    ret
}

/// The path to the job module's Rust source file.
pub fn job_ref_file(job: JobRef) -> std::path::PathBuf {
    let mut ret = jobs_dir();
    ret.push(job_ref_module_name(job));
    ret.with_added_extension(RUST_EXT)
}

/// Create the name of the structure that holds the job module instance.
pub fn job_ref_mod_struct(job: JobRef) -> String {
    format!("Job{}", job)
}

/// Create the name of the job structure::job::JobRunner implementation.
pub fn job_ref_run_struct(job: JobRef) -> String {
    format!("Job{}Runner", job)
}

/// Create the structure for the job event handler.
pub fn event_handler_struct(job: JobRef, handler: usize) -> String {
    format!("Job{}Event{}", job, handler)
}

/// The field in the runtime structure containing the job wrapper.
pub fn job_runtime_field(job: JobRef) -> String {
    format!("j{}", job)
}

/// Construct a unique, rust-compatible name from the string.
/// While not easy to read, it's guaranteed to be 1-to-1 unique with the
/// input name.
pub(crate) fn unique_name(name: &String) -> String {
    let mut ret = String::new();
    for c in name.chars() {
        // In order to keep this alphanumeric + unique,
        // replace these names with a special encoding.
        if c.is_ascii_alphanumeric() {
            ret.push(c);
        } else if c == '_' {
            ret.push('_');
            ret.push('_');
        } else {
            ret.push('_');
            let c: u32 = c.into();
            ret.push_str(format!("{:x}", c).as_str());
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_name() {
        let test_data = [
            ("", ""),
            ("a", "a"),
            ("B", "B"),
            ("a_b", "a__b"),
            ("a b", "a_20b"),
            ("%a $6", "_25a_20_246"),
        ];
        for (inp, exp) in test_data {
            assert_eq!(exp, unique_name(&inp.into()));
        }
    }
}
