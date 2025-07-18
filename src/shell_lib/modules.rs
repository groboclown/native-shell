pub mod cat;
pub mod cp;
pub mod echo;
pub mod file_sink;
pub mod merge;
pub mod shell;
pub mod tee;

/// Get all available modules for use by scripts.
pub fn available_modules() -> Vec<super::compile::meta::ModuleMeta> {
    vec![
        cat::module_meta(),
        cp::module_meta(),
        file_sink::module_meta(),
        shell::module_meta(),
    ]
}
