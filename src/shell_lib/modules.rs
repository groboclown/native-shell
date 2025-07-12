pub mod cat;
pub mod cp;
pub mod file_sink;

/// Get all available modules for use by scripts.
pub fn available_modules() -> Vec<super::compile::meta::ModuleMeta> {
    vec![
        cat::module_meta(),
        cp::module_meta(),
        file_sink::module_meta(),
    ]
}
