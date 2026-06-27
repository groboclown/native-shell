//! Turns the AST into a module source.

use std::rc::Rc;

use super::sequence::StdSequenceStore;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::{BuilderError, ErrorDetails};
use crate::server_shell::lls::model;
use crate::shell_lib::modules;
use crate::shell_lib::structure::meta;

const AST_VERSION_1: &str = "1.0.0";

pub fn ast_to_module_source<SW: SourceWriter>(
    ast: &model::NativeShellAstSchema,
    out: SW,
) -> Result<(), BuilderError> {
    check_supported(ast)?;

    // Step 1: turn the ast into the internal representation.
    let module_nodes = super::parse_node::convert_nodes(&ast.nodes, &get_available_modules())?;

    // Step 2: Construct the node graphs.
    let stream_graphs = super::node_graph::ScriptGraph::load(&module_nodes)?;

    // Step 3: Create the Cargo.toml file.
    super::gen_cargo::write_cargo_toml(&ast.name, &ast.version, &module_nodes, &out)?;

    // Step 4: Write the runtime.rs file.
    super::gen_runtime::write_runtime_rs(&module_nodes, &out)?;

    let seq = StdSequenceStore::new(module_nodes, stream_graphs);

    // Step 5: Write the seq*.rs files.
    //   There are two types of sequences: the graph based ones
    //   and the ordered action based ones.  Initially, generate the
    //   graph based ones.
    for (idx, graph) in seq.graphs().iter().enumerate() {
        super::gen_seq_graph::write_graph_seq(idx, graph, &seq, &out)?;
    }
    for (idx, actions) in seq.ordered_sequences() {
        super::gen_seq_ordered::write_ordered_seq(idx, &actions, &out)?;
    }

    // Step 6: Write the main.rs file.
    super::gen_main::write_main_rs(&seq, &out)?;

    // Step 7: Copy the shell_lib into the output directory.
    //   This can either be an unzip step, or it can be provided
    //   in the executable's install directory.
    //   For now, it's hard-coded.
    super::extract_lib::extract_libs(&out)?;

    Ok(())
}

pub fn check_supported(ast: &model::NativeShellAstSchema) -> Result<(), BuilderError> {
    if ast.schema_version != AST_VERSION_1 {
        Err(BuilderError::InvalidAst(ErrorDetails {
            message: format!(
                "Unsupported AST version: {}. Supported versions include: {}",
                ast.schema_version, AST_VERSION_1
            ),
            source: ast.source.clone(),
            related: vec![],
        }))
    } else {
        Ok(())
    }
}

pub fn get_available_modules() -> Vec<Rc<meta::ModuleMeta>> {
    let mut ret = Vec::new();
    for m in modules::available_modules() {
        ret.push(Rc::new(m));
    }
    ret
}
