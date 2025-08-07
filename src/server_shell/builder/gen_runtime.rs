//! Create the runtime file.

use super::helpers::{as_mod_expr, rust_file_header};
use super::parse_node::ModuleNode;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::BuilderError;

/// Generate the shared runtime structures.
pub fn write_runtime_rs<SW: SourceWriter>(
    nodes: &Vec<ModuleNode>,
    out: &SW,
) -> Result<(), BuilderError> {
    let mut out = out.writer_for("src/runtime.rs")?;
    out.write_all(rust_file_header().as_bytes())?;
    out.write_all(
        b"use std::sync::Arc;\n//use crate::shell_lib::helpers;\n\npub struct Nodes {\n",
    )?;
    for node in nodes {
        out.write_fmt(format_args!(
            "    pub {}: {}{},\n",
            node.node_id,
            as_mod_expr(node),
            node.module.instance_struct,
        ))?;
    }

    /* For the moment, runtime parameters are built by the jobs on-demand.
    out.write_all(b"}\n\npub struct RuntimeParams {\n")?;
    for node in nodes {
        if let Some(rt) = &node.module.runtime_param_struct {
            out.write_fmt(format_args!(
                "    pub {}: {}{},\n",
                node.node_id,
                as_mod_expr(node),
                rt.name,
            ))?;
        }
    }
    out.write_all(b"}\n\n#[derive(Clone)]\npub struct Runtime {\n    pub nodes: Arc<Nodes>,\n    pub params: helpers::state_guard::StateGuard<RuntimeParams>,\n}\n")?;
    */
    out.write_all(b"}\n\n#[derive(Clone)]\npub struct Runtime {\n    pub nodes: Arc<Nodes>,\n}\n")?;

    Ok(())
}
