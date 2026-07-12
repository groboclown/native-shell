//! Turns the AST into a module source.

use std::rc::Rc;

use super::sequence::StdSequenceStore;
use super::writer::SourceWriter;
use crate::server_shell::builder::errors::{BuilderError, ErrorDetails};
use crate::server_shell::lls::model;
use crate::shell_lib::modules;
use crate::shell_lib::structure::meta;

const LLS_VERSION_1: &str = "1.0.0";

pub fn lls_to_module_source<SW: SourceWriter>(
    lls: &model::NativeShellLowLevelScriptSchema,
    out: SW,
) -> Result<(), BuilderError> {
    check_supported(lls)?;

    // Step 1: turn the LLS into the internal representation.
    let module_nodes =
        super::assemble::convert(&lls, &get_available_modules(), &get_available_macros())?;

    // Step 2: Construct the node graphs.
    // The new LLS makes this obsolete.  This is now the responsibility
    // of the script parser.
    //let stream_graphs = super::node_graph::ScriptGraph::load(&module_nodes)?;

    // Step 3: Create the Cargo.toml file.
    super::gen_cargo::write_cargo_toml(&lls.meta.name, &lls.meta.version, &module_nodes, &out)?;

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

pub fn check_supported(lls: &model::NativeShellLowLevelScriptSchema) -> Result<(), BuilderError> {
    if lls.schema_version != LLS_VERSION_1 {
        Err(BuilderError::InvalidAst(ErrorDetails {
            message: format!(
                "Unsupported LLS version: {}. Supported versions include: {}",
                lls.schema_version, LLS_VERSION_1
            ),
            source: lls.meta.source.clone(),
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

pub fn get_available_macros() -> Vec<Rc<Box<dyn super::super::meta::MacroMeta>>> {
    let mut ret = Vec::new();
    for m in super::super::macros::available_macros() {
        ret.push(Rc::new(m));
    }
    ret
}
