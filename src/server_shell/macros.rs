//SPDX:MIT

//! All built-in macros.
pub mod shell;

pub fn available_macros() -> Vec<Box<dyn super::meta::MacroMeta>> {
    vec![shell::macro_meta()]
}
