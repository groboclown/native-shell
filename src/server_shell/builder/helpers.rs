//! Helpers for the rust file generation.

use crate::server_shell::builder::parse_node::ModuleNode;

/// Create the node's module's full crate and module name.
pub fn as_mod_expr(node: &ModuleNode) -> String {
    let mut line = "crate".to_owned();
    for item in &node.module.mod_name {
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
