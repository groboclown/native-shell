//! Generate the Cargo.toml file for a script.

use std::collections::{HashMap, HashSet};

use super::helpers::toml_file_header;
use super::parse_node::ModuleNode;
use super::shell_lib_deps::SHELL_LIB_DEPS;
use super::writer::SourceWriter;
use crate::server_shell::builder::cargo::cargo_lib::get_crate_dependency;
use crate::server_shell::builder::errors::BuilderError;
use crate::shell_lib::structure::meta::{CrateDependency, VerBit};

pub fn write_cargo_toml<SW: SourceWriter>(
    name: &String,
    version: &String,
    nodes: &Vec<ModuleNode>,
    out: &SW,
) -> Result<(), BuilderError> {
    let mut out = out.writer_for("Cargo.toml")?;
    out.write_all(toml_file_header().as_bytes())?;
    out.write_all(b"[package]\n")?;
    out.write_fmt(format_args!("name = \"{}\"\n", name))?;
    out.write_fmt(format_args!("version = \"{}\"\n", version))?;
    out.write_all(b"edition = \"2024\"\n\n[dependencies]\n")?;
    for dep in distinct_dependencies(nodes)? {
        out.write_all(dep.as_bytes())?;
    }
    for (target, deps) in distinct_target_dependencies(nodes) {
        out.write_all(b"\n")?;
        out.write_fmt(format_args!(
            "[target.'cfg(target_os = \"{}\")'.dependencies\n",
            target
        ))?;
        for dep in deps {
            out.write_fmt(format_args!("{}\n", dep))?;
        }
    }
    Ok(())
}

/// Extract the dependencies from the modules.
fn distinct_dependencies(nodes: &Vec<ModuleNode>) -> Result<Vec<String>, BuilderError> {
    // This implementation is very inefficient for memory usage.
    let mut best_deps = HashMap::new();
    let mut defaults = HashMap::new();
    for dep_name in SHELL_LIB_DEPS.iter() {
        let dep = get_crate_dependency(dep_name)?;
        defaults.insert(dep.name.clone(), dep.clone());
        best_deps.insert(dep.name.clone(), dep.clone());
    }

    for node in nodes {
        for dep in &node.module.dependencies {
            let dep = match dep.version.len() {
                0 => match defaults.get(&dep.name) {
                    Some(d) => d,
                    None => {
                        let d = get_crate_dependency(&dep.name)?;
                        defaults.insert(d.name.clone(), d.clone());
                        &d.clone()
                    }
                },
                _ => dep,
            };
            match best_deps.get(&dep.name) {
                None => {
                    best_deps.insert(dep.name.clone(), dep.clone());
                }
                Some(existing) => {
                    let selected = select_highest_version(existing, dep);
                    best_deps.insert(dep.name.clone(), selected.clone());
                }
            }
        }
    }
    let mut ret = Vec::new();
    for dep in best_deps.values() {
        ret.push(as_cargo_line(dep));
    }
    Ok(ret)
}

fn distinct_target_dependencies(nodes: &Vec<ModuleNode>) -> Vec<(String, Vec<String>)> {
    let mut os: HashMap<String, HashSet<String>> = HashMap::new();
    for node in nodes {
        for (target, dep) in &node.module.os_dependencies {
            match os.get_mut(target) {
                Some(v) => {
                    v.insert(dep.clone());
                }
                None => {
                    let mut val = HashSet::new();
                    val.insert(dep.clone());
                    os.insert(target.clone(), val);
                }
            };
        }
    }
    let mut ret = Vec::with_capacity(os.len());
    for (target, deps) in os.into_iter() {
        if !deps.is_empty() {
            let deps: Vec<String> = deps.into_iter().collect();
            ret.push((target, deps));
        }
    }
    ret
}

fn select_highest_version<'a>(
    left: &'a CrateDependency,
    right: &'a CrateDependency,
) -> &'a CrateDependency {
    let len = std::cmp::min(left.version.len(), right.version.len());
    for i in 0..len {
        // Numbers have a higher precedence than strings.
        match (&left.version[i], &right.version[i]) {
            (VerBit::N(ln), VerBit::N(rn)) => {
                match compare_version_separator(left, ln.0, right, rn.0) {
                    Some(res) => return res,
                    None => {
                        if ln.1 > rn.1 {
                            return left;
                        } else if rn.1 > ln.1 {
                            return right;
                        }
                        // else keep searching
                    }
                }
            }
            (VerBit::S(ls), VerBit::N(rn)) => {
                match compare_version_separator(left, ls.0, right, rn.0) {
                    Some(res) => return res,
                    None => {
                        return right;
                    }
                }
            }
            (VerBit::N(ln), VerBit::S(rs)) => {
                match compare_version_separator(left, ln.0, right, rs.0) {
                    Some(res) => return res,
                    None => {
                        return left;
                    }
                }
            }
            (VerBit::S(ls), VerBit::S(rs)) => {
                match compare_version_separator(left, ls.0, right, rs.0) {
                    Some(res) => return res,
                    None => {
                        // In general, lexical comparison works fine.
                        // This covers 'a' vs 'b' and 'alpha' vs 'beta', etc.
                        // It doesn't cover 'rc' vs 'ga', though.
                        if ls > rs {
                            return left;
                        } else if rs > ls {
                            return right;
                        }
                    }
                }
            }
        }
    }
    if left.version.len() >= right.version.len() {
        return left;
    }
    return right;
}

fn compare_version_separator<'a>(
    left: &'a CrateDependency,
    lc: char,
    right: &'a CrateDependency,
    rc: char,
) -> Option<&'a CrateDependency> {
    if lc != rc {
        if lc == '.' {
            return Some(left);
        } else if rc == '.' {
            return Some(right);
        } else if lc > rc {
            // Both are non-dot separators, compare lexically.
            // By using >, it means \0 is always lowest precedence.
            return Some(left);
        }
        // This does not include ==, because this starts with 'if ln.0 != rn.0'.
        return Some(right);
    }
    None
}

fn as_cargo_line(dep: &CrateDependency) -> String {
    let mut line = format!("{} = ", dep.name);
    if dep.features.len() > 0 {
        line.push_str("{ version = \"");
    } else {
        line.push('"');
    }
    for bit in &dep.version {
        match bit {
            VerBit::S((sep, s)) => {
                if *sep != '\0' {
                    line.push(*sep);
                }
                line.push_str(s);
            }
            VerBit::N((sep, n)) => {
                if *sep != '\0' {
                    line.push(*sep);
                }
                line.push_str(&n.to_string());
            }
        }
    }
    if dep.features.len() > 0 {
        line.push_str("\", features = [");
        for (i, feat) in dep.features.iter().enumerate() {
            if i > 0 {
                line.push_str(", ");
            }
            line.push('"');
            line.push_str(feat);
            line.push('"');
        }
        line.push_str("] }\n");
    } else {
        line.push_str("\"\n");
    }
    line
}
