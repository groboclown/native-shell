//! Parse the AST nodes into usable versions.

use std::rc::Rc;

use crate::{server_shell::{ast::model::Node, builder::errors}, shell_lib::compile::meta};


pub struct ModuleNode {
    pub node: Node,
    pub module: Rc<meta::ModuleMeta>,
    pub module_id: String,
    pub src_streams: Vec<NodeStream>,
    pub dest_streams: Vec<NodeStream>,
}

impl ModuleNode {
    pub fn new(node: Node, module: Rc<meta::ModuleMeta>, module_id: String) -> Self {
        ModuleNode {
            node,
            module,
            module_id,
            src_streams: Vec::new(),
            dest_streams: Vec::new(),
        }
    }

    pub fn add_src_stream(&mut self, stream: NodeStream) {
        self.src_streams.push(stream);
    }

    pub fn add_dest_stream(&mut self, stream: NodeStream) {
        self.dest_streams.push(stream);
    }
}

pub fn convert_nodes(nodes: &Vec<Node>, modules: &Vec<Rc<meta::ModuleMeta>>) -> (Vec<ModuleNode>, Vec<errors::BuilderError>) {
    let mut module_nodes = Vec::new();
    let mut errors = Vec::new();

    for (id, node) in nodes.iter().enumerate() {
        let module = match find_module_by_name(&node.module, modules) {
            Ok(module) => module,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        let mod_id = "module_".to_string() + &id.to_string();
        module_nodes.push(ModuleNode::new(node.clone(), module, mod_id.clone()));
    }
    (module_nodes, errors)
}

pub fn find_module_by_name(
    module_name: &str,
    modules: &Vec<Rc<meta::ModuleMeta>>,
) -> Result<Rc<meta::ModuleMeta>, errors::BuilderError> {
    for module in modules {
        if module.name == module_name {
            return Ok(module.clone());
        }
    }
    Err(errors::BuilderError::ModuleNotFound(module_name.to_string()))
}


pub struct NodeStream {
    pub source_id: String,
    pub dest_id: String,
    pub source_type: meta::StreamType,
    pub source_decl: meta::StreamDeclaration,
    pub dest_type: meta::StreamType,
    pub dest_decl: meta::StreamDeclaration,
}

impl NodeStream {
    pub fn new(
        source_id: String,
        dest_id: String,
        source_type: meta::StreamType,
        source_decl: meta::StreamDeclaration,
        dest_type: meta::StreamType,
        dest_decl: meta::StreamDeclaration,
    ) -> Self {
        NodeStream {
            source_id,
            dest_id,
            source_type,
            source_decl,
            dest_type,
            dest_decl,
        }
    }
}
