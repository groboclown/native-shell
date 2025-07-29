//! Turns the AST into a module source.

use std::rc::Rc;

use crate::server_shell::ast::model;
use crate::server_shell::builder::errors::BuilderError;
use crate::shell_lib::compile::meta;
use crate::shell_lib::modules;
use super::writer::SourceWriter;
use super::parse_node::{ModuleNode, NodeStream};

const AST_VERSION_1: &str = "1.0.0";


pub fn ast_to_module_source<SW: SourceWriter>(
    ast: &model::NativeShellAstSchema,
    out: SW,
) -> Result<(), String> {
    check_supported(ast)?;

    // Step 1: turn the ast into the internal representation.
    let module_nodes = match super::parse_node::convert_nodes(&ast.nodes, &get_available_modules()) {
        (nodes, errors) => {
            if !errors.is_empty() {
                report_errors(errors);
                return Err("Errors found in the script.".to_string());
            }
            nodes
        }
    };

    // Step 2: Construct the node graphs.
    // Step 3: Create the Cargo.toml file.
    // Step 4: Write the runtime.rs file.
    // Step 5: Write the seq*.rs files.
    // Step 6: Write the main.rs file.

    // out.write(join_uses(ast))?;
    todo!("Implement the rest of the steps to convert the AST to module source");
}

/// Create the module's 'use' lines.
pub fn join_uses(nodes: Vec<ModuleNode>) -> String {
    "not implemented".to_string()
    /*
    let mut ret = HashSet::new();
    for part in ast.nodes.iter() {
        if part.is_empty() {
            continue;
        }
        let mut joined = "use ".to_string();
        joined.push_str(part.join("::").as_str());
        joined.push_str(";\n");
        ret.insert(joined);
    }
    return ret.into_iter()
        .collect::<Vec<String>>()
        .join("");
    */
}

pub fn check_supported(ast: &model::NativeShellAstSchema) -> Result<(), String> {
    if ast.version != AST_VERSION_1 {
        return Err(format!(
            "Unsupported AST version: {}. Supported versions include: {}",
            ast.version,
            AST_VERSION_1,
        ));
    }
    Ok(())
}

fn get_available_modules() -> Vec<Rc<meta::ModuleMeta>> {
    let mut ret = Vec::new();
    for m in modules::available_modules() {
        ret.push(Rc::new(m));
    }
    ret
}

fn report_errors(errs: Vec<BuilderError>) {
    for err in errs {
        match err {
            BuilderError::InvalidAst(details) => {
                eprintln!("Invalid AST: {}", details.message);
                eprintln!("Source: {:?}", details.source);
                for related in details.related {
                    eprintln!("Related: {:?}", related);
                }
            }
            BuilderError::ModuleNotFound(details) => {
                eprintln!("Module not found: {}", details.message);
                eprintln!("Source: {:?}", details.source);
                for related in details.related {
                    eprintln!("Related: {:?}", related);
                }
            }
            BuilderError::NoSuchNode(details) => {
                eprintln!("No such node: {}", details.message);
                eprintln!("Source: {:?}", details.source);
                for related in details.related {
                    eprintln!("Related: {:?}", related);
                }
            }
            BuilderError::StreamNotFound(details) => {
                eprintln!("Stream not found: {}", details.message);
                eprintln!("Source: {:?}", details.source);
                for related in details.related {
                    eprintln!("Related: {:?}", related);
                }
            }
            BuilderError::MultipleStreamUse(details) => {
                eprintln!("Multiple stream use: {}", details.message);
                eprintln!("Source: {:?}", details.source);
                for related in details.related {
                    eprintln!("Related: {:?}", related);
                }
            }
            BuilderError::IOError(e) => {
                eprintln!("I/O error: {}", e);
            }
        }
    }
}
