//! Helpers for the rust file generation.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    server_shell::{builder::errors, lls},
    shell_lib::structure,
};

pub const SOURCE_ABS: &'static str = "crate::shell_lib::structure::Source";

/// Write a String to the output, passing errors to the issue set.
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

/// Write a &String to the output, passing errors to the issue set.
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

/// Write a &str to the output, passing errors to the issue set.
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

const _MOD_JOIN: &str = "::";

/// Qualify the name with the module parts.
/// Does not perform any 'crate::' prefix.
/// Note that, to make construction of the file easier, rather than easier to read,
/// the recommended approach has fully qualified names for all references, rather than
/// including a 'use' section.
pub fn qualify_name(module: &Vec<String>, name: &String) -> String {
    let mut ret = module.join(_MOD_JOIN);
    ret.push_str(_MOD_JOIN);
    ret.push_str(name.as_str());
    ret
}

const SCRIPT_EXIT_ABS: &'static str = "crate::shell_lib::structure::ScriptExit";

/// Create the ScriptExit object, as rust code, for reporting an error.
pub fn script_exit<'a, 'b, S: Into<String>>(
    source: &'a lls::model::Source,
    exit_code: structure::ExitCode,
    message: S,
) -> String {
    let source = structure::Source::new(
        source.file.as_str(),
        source.line.unwrap_or(0),
        source.column.unwrap_or(0),
    );
    format!(
        "{}::new({}, Some({}))",
        SCRIPT_EXIT_ABS,
        exit_code,
        as_rust_str(&format!("{}: {}", source, message.into()))
    )
}

pub fn return_script_exit<'a, 'b, S: Into<String>>(
    source: &'a lls::model::Source,
    exit_code: structure::ExitCode,
    message: S,
) -> String {
    format!("return Err({});", script_exit(source, exit_code, message))
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

/// Convert the string list into a Rust Vec<String>.
pub fn as_rust_string_list<T: Into<String> + Clone>(list: &Vec<T>) -> String {
    as_rust_list(list, |v| as_rust_string(&v.clone().into()))
}

/// Convert a boolean into a Rust boolean literal.
pub fn as_rust_bool(val: bool) -> &'static str {
    match val {
        true => "true",
        false => "false",
    }
}

/// Convert the string list into a Rust Vec<String>.
pub fn as_rust_bool_list<T: Into<bool> + Clone>(list: &Vec<T>) -> String {
    as_rust_list(list, |v| as_rust_bool(v.clone().into()).to_string())
}

/// Convert the floating point number into a Rust number.
pub fn as_rust_number(val: f64) -> String {
    // May want to make this more robust.
    val.to_string()
}

/// Convert the floating point number into a Rust number.
pub fn as_rust_float(val: f64) -> String {
    // May want to make this more robust.
    val.to_string()
}

/// Convert the string list into a Rust Vec<String>.
pub fn as_rust_float_list<T: Into<f64> + Clone>(list: &Vec<T>) -> String {
    as_rust_list(list, |v| as_rust_float(v.clone().into()))
}

/// Convert the list of items (using the conversion function) into a list.
pub fn as_rust_list<T>(list: &Vec<T>, conv: fn(&T) -> String) -> String {
    let mut ret = "vec![".to_string();
    let mut first = true;
    for s in list {
        if first {
            first = false;
        } else {
            ret.push(',');
            ret.push(' ');
        }
        ret.push_str(conv(s).as_str());
    }
    ret.push(']');
    ret
}

/// Convert the map into a hashmap.
pub fn as_rust_map<K: Into<String> + Clone, V>(
    map: &HashMap<K, V>,
    conv: fn(&V) -> String,
) -> String {
    let mut ret = "[".to_string();
    let mut first = true;
    for (k, v) in map {
        if first {
            first = false
        } else {
            ret.push(',');
            ret.push(' ');
        }
        ret.push('(');
        ret.push_str(as_rust_string(&k.clone().into()).as_str());
        ret.push(',');
        ret.push_str(conv(v).as_str());
        ret.push(')');
    }
    ret
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

/// Create the SPDX code from the metadata license.
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

/// Get the current UTC date/time in RFC 3339 format.
pub fn utc_now() -> String {
    let current_utc = chrono::Utc::now();
    current_utc.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualify_name() {
        let test_data: &[(&[&str], &str, &str)] = &[
            (&["a"], "b", "a::b"),
            (&["abc", "def"], "hij", "abc::def::hij"),
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
