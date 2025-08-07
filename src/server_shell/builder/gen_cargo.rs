//! Generate the Cargo.toml file for a script.

use std::collections::{HashMap, HashSet};

use super::helpers::toml_file_header;
use super::parse_node::ModuleNode;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::BuilderError;

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
    for dep in distinct_dependencies(nodes) {
        out.write_fmt(format_args!("{}\n", dep))?;
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
fn distinct_dependencies(nodes: &Vec<ModuleNode>) -> Vec<String> {
    let mut ret = HashSet::new();
    for node in nodes {
        for dep in &node.module.dependencies {
            ret.insert(dep.clone());
        }
    }
    return ret.into_iter().collect();
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
