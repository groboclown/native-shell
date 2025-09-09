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
    out.write_fmt(format_args!(
        "use std::sync::Arc;\nuse std::collections::HashMap;\n//use crate::shell_lib::helpers;\nuse {};\n\npub struct Nodes {{\n",
        super::helpers::SOURCE_MODULE,
    ))?;
    for node in nodes {
        out.write_fmt(format_args!(
            "    pub {}: {}{},\n",
            node.node_id,
            as_mod_expr(node),
            node.module.instance_struct,
        ))?;
    }
    out.write_all(
        b"}

#[derive(Clone)]
pub struct Runtime {
    pub nodes: Arc<Nodes>,
}

impl Runtime {
    pub fn new(argv: Vec<String>, environ: HashMap<String, String>) -> Self {
        Runtime {
            nodes: Arc::new(Nodes::new(argv, environ)),
        }
    }
}

impl Nodes {
    pub fn new(argv: Vec<String>, environ: HashMap<String, String>) -> Self {
"
    )?;

    for node in nodes {
        if let Some(params) = &node.module.compile_param_struct {
            if let Some(new) = &params.new {
                out.write_fmt(format_args!(
                    "        let mut params_{} = {}{}::{}();\n",
                    node.node_id, as_mod_expr(node), params.name, new,
                ))?;
                let field_values = super::values::construct_parameter_values(
                    &node.node.source,
                    &node.node.initial_parameters,
                    params,
                )?;
                for (key, field) in field_values {
                    out.write_fmt(format_args!(
                        "        params_{}.{} = {};\n",
                        node.node_id, key, field,
                    ))?;
                }
            }
        }
    }

    // Generate the return statement.
    out.write_all(
        b"\n        Nodes {\n",
    )?;
    for node in nodes {
        out.write_fmt(format_args!(
            "            {}: {}{}::new({}",
            node.node_id,
            as_mod_expr(node),
            node.module.instance_struct,
            super::helpers::as_rust_source(&node.node.source),
        ))?;
        if let Some(params) = &node.module.compile_param_struct {
            if params.new.is_some() {
                out.write_fmt(format_args!(
                    ", params_{}",
                    node.node_id,
                ))?;
            } else {
                out.write_fmt(format_args!(", {}{} {{\n",
                    as_mod_expr(node), params.name,
                ))?;
                let field_values = super::values::construct_parameter_values(
                    &node.node.source,
                    &node.node.initial_parameters,
                    params,
                )?;
                for (key, field) in field_values {
                    out.write_fmt(format_args!(
                        "                {}: {},\n",
                        key, field,
                    ))?;
                }
                out.write_all(b"}\n")?;
            }
        }
        out.write_all(b"),\n")?;
    }
    out.write_all(
        b"        }\n    }\n}\n",
    )?;

    Ok(())
}
