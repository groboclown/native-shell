//SPDX:MIT

//! All built-in macros.

use std::collections::HashMap;
pub mod shell;

pub fn available_macros() -> HashMap<String, Box<dyn super::meta::MacroMeta + Send + Sync>> {
    let mut ret = HashMap::new();
    ret.insert("shell".into(), shell::macro_meta());
    ret
}
