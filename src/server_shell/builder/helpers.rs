//! Helpers for the rust file generation.

use crate::{
    server_shell::{builder::parse_node::ModuleNode, lls::model},
    shell_lib::structure::meta,
};

/// Create the node's module's full crate and module name.
pub fn as_mod_expr(node: &ModuleNode) -> String {
    module_as_mod_expr(&node.module)
}

/// Create the node's module's full crate and module name.
pub fn module_as_mod_expr(module: &meta::ModuleMeta) -> String {
    let mut line = "crate".to_owned();
    for item in &module.mod_name {
        line.push_str("::");
        line.push_str(&item);
    }
    line.push_str("::");
    line
}

/// Create the rust file header.
pub fn rust_file_header() -> String {
    let current_utc = chrono::Utc::now();
    format!(
        "//! GENERATED FILE.  DO NOT EDIT.\n//! Created on {}\n\n",
        current_utc.to_rfc3339(),
    )
}

/// Create the toml file header.
pub fn toml_file_header() -> String {
    let current_utc = chrono::Utc::now();
    format!(
        "# GENERATED FILE.  DO NOT EDIT.\n# Created on {}\n\n",
        current_utc.to_rfc3339(),
    )
}

/// Convert a String to a Rust string literal.
pub fn as_rust_str(text: &String) -> String {
    let mut s = "\"".to_string();
    for c in text.chars() {
        match c {
            '"' => s.push_str("\\\""),
            '\\' => s.push_str("\\\\"),
            '\n' => s.push_str("\\n"),
            '\r' => s.push_str("\\r"),
            '\t' => s.push_str("\\t"),
            '\0' => s.push_str("\\0"),
            _ => s.push(c),
        }
    }
    s.push('\"');
    s
}

/// Convert a String to a Rust string literal.
pub fn as_rust_string(text: &String) -> String {
    let mut s = as_rust_str(text);
    s.push_str(".to_string()");
    s
}

/// Convert a Source to a Rust source expression.
pub fn as_rust_source(source: &model::Source) -> String {
    let mut s = "Source::new(".to_string();
    s.push_str(&as_rust_str(&source.file));
    s.push_str(", ");
    s.push_str(&source.line.to_string());
    s.push_str(", ");
    s.push_str(&source.column.to_string());
    s.push_str(")");
    s
}

pub const SOURCE_MODULE: &str = "crate::shell_lib::compile::source::Source";
