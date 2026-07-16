//! Helpers for the rust file generation.

use std::sync::Arc;

use crate::server_shell::{builder::errors, lls};

pub const SOURCE_MODULE: &str = "crate::shell_lib::structure::Source";

pub fn write_string<'a, 'b>(
    out: &'a mut Box<dyn std::io::Write>,
    issues: &'b errors::ScriptIssues,
    text: String,
) -> Result<(), ()> {
    issues.consume(
        out.write(text.as_bytes())
            .map(|_| ())
            .map_err(|e| errors::BuilderError::from(e)),
    )
}

pub fn write_string_ref<'a, 'b, 'c>(
    out: &'a mut Box<dyn std::io::Write>,
    issues: &'b errors::ScriptIssues,
    text: &'c String,
) -> Result<(), ()> {
    issues.consume(
        out.write(text.as_bytes())
            .map(|_| ())
            .map_err(|e| errors::BuilderError::from(e)),
    )
}

pub fn write_str<'a, 'b, 'c>(
    out: &'a mut Box<dyn std::io::Write>,
    issues: &'b errors::ScriptIssues,
    text: &'c str,
) -> Result<(), ()> {
    issues.consume(
        out.write(text.as_bytes())
            .map(|_| ())
            .map_err(|e| errors::BuilderError::from(e)),
    )
}

/// Convert the module name pieces into a module name.
/// Usable in a 'use mod' statement or explicit qualification.
pub fn as_mod_expr(module: &Vec<String>) -> String {
    assert!(!module.is_empty());
    let mut ret = "crate::".to_owned();
    ret.push_str(module.join("::").as_str());
    ret
}

/// Qualify the name with the module parts.
/// Does not perform any 'crate::' prefix.
pub fn qualify_name(module: &Vec<String>, name: &String) -> String {
    let mut ret = module.join("::");
    ret.push_str("::");
    ret.push_str(name.as_str());
    ret
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

/// Create the rust file header.
pub fn rust_file_header(meta: &Arc<lls::model::Metadata>, now: &String) -> String {
    format!(
        "{}//! GENERATED FILE.  DO NOT EDIT.\n//! Created on {}\n\n",
        get_spdx_code(meta, "//", "\n"),
        now,
    )
}

/// Create the toml file header.
pub fn toml_file_header(meta: &Arc<lls::model::Metadata>, now: &String) -> String {
    format!(
        "{}# GENERATED FILE.  DO NOT EDIT.\n# Created on {}\n\n",
        get_spdx_code(meta, "#", "\n"),
        now,
    )
}

fn get_spdx_code(meta: &Arc<lls::model::Metadata>, prefix: &str, suffix: &str) -> String {
    match &meta.license {
        None => String::new(),
        Some(lic) if lic.spdx_id.is_empty() => String::new(),
        Some(lic) => {
            let mut ret = prefix.to_string();
            ret.push_str("SPDX:");
            ret.push_str(lic.spdx_id.as_str());
            ret.push_str(suffix);
            ret
        }
    }
}

pub fn utc_now() -> String {
    let current_utc = chrono::Utc::now();
    current_utc.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_mod_expr() {
        let test_data: &[(&[&str], &str)] =
            &[(&["a"], "crate::a"), (&["abc", "def"], "crate::abc::def")];
        for (inp, exp) in test_data {
            assert_eq!(as_mod_expr(&as_vec_string(inp)), exp.to_string());
        }
    }

    #[test]
    fn test_qualify_name() {
        let test_data: &[(&[&str], &str, &str)] = &[
            (&["a"], "b", "crate::a::b"),
            (&["abc", "def"], "hij", "crate::abc::def::hij"),
        ];
        for (pak, name, exp) in test_data {
            assert_eq!(
                qualify_name(&as_vec_string(pak), &name.to_string()),
                exp.to_string()
            );
        }
    }

    #[test]
    fn test_as_rust_str() {
        let test_data: &[(&str, &str)] = &[
            ("", "\"\""),
            ("a'b", "\"a'b\""),
            ("a\"b", "\"a\\\"b\""),   // a"b -> "a\"b"
            ("\\\"", "\"\\\\\\\"\""), // \\" -> "\\\""
        ];
        for (inp, exp) in test_data {
            assert_eq!(as_rust_str(&inp.to_string()), exp.to_string());
        }
    }

    fn as_vec_string<'a, 'b>(i: &'a [&'b str]) -> Vec<String> {
        let mut ret = Vec::new();
        for s in i {
            ret.push(s.to_string());
        }
        ret
    }
}

/*
use crate::{
    server_shell::{builder::assemble::ModuleNode, lls::model},
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

/// Convert a Source to a Rust source expression.
pub fn as_rust_source(source: &model::Source) -> String {
    format!(
        "Source::new({}, {}, {})",
        as_rust_str(&source.file),
        match &source.line {
            Some(s) => s,
            None => &(0 as i64),
        },
        match source.column {
            Some(s) => s,
            None => 0,
        }
    )
}

*/
