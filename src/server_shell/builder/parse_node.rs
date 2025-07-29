//! Parse the AST nodes into usable versions.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::server_shell::ast::model;
use crate::server_shell::builder::errors;
use crate::shell_lib::compile::meta;


pub struct ModuleNode {
    pub node: model::Node,
    pub module: Rc<meta::ModuleMeta>,
    pub node_id: String,
    pub src_streams: Vec<Rc<NodeStream>>,
    pub dest_streams: Vec<Rc<NodeStream>>,
}

impl ModuleNode {
    pub fn new(node: model::Node, module: Rc<meta::ModuleMeta>, node_id: String) -> Self {
        ModuleNode {
            node,
            module,
            node_id,
            src_streams: Vec::new(),
            dest_streams: Vec::new(),
        }
    }

    pub fn add_src_stream(&mut self, stream: Rc<NodeStream>) {
        self.src_streams.push(stream);
    }

    pub fn add_dest_stream(&mut self, stream: Rc<NodeStream>) {
        self.dest_streams.push(stream);
    }
}

pub fn convert_nodes(nodes: &Vec<model::Node>, modules: &Vec<Rc<meta::ModuleMeta>>) -> (Vec<ModuleNode>, Vec<errors::BuilderError>) {
    let mut nodes_by_name = HashMap::new();
    let module_nodes = Vec::new();
    let mut errors = Vec::new();

    // First pass: gather the nodes.
    for (id, ast_node) in nodes.iter().enumerate() {
        let module = match find_module_by_name(&ast_node.source, &ast_node.module, modules) {
            Ok(module) => module,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        let mod_id = "node_".to_string() + &id.to_string();
        let node = ModuleNode::new(ast_node.clone(), module, mod_id);
        nodes_by_name.insert(ast_node.name.clone(), RefCell::new(node));
    }
    
    // Second pass: associate the streams with the nodes.
    for node in nodes_by_name.values() {
        let node = node;
        let streams = &node.borrow().node.streams.clone();
        for stream in streams {
            setup_node_stream(node, &stream, &nodes_by_name);
        }
    }

    // TODO next step: ensure the required streams exist, and there are no duplicates, and
    // variable streams meet minimum and maximum requirements.

    (module_nodes, errors)
}

pub fn find_module_by_name(
    source: &model::Source,
    module_name: &str,
    modules: &Vec<Rc<meta::ModuleMeta>>,
) -> Result<Rc<meta::ModuleMeta>, errors::BuilderError> {
    for module in modules {
        if module.name == module_name {
            return Ok(module.clone());
        }
    }
    Err(errors::BuilderError::ModuleNotFound(errors::ErrorDetails{
        message: module_name.to_string(),
        source: source.clone(),
        related: vec![],
    }))
}

fn setup_node_stream(
    source: &RefCell<ModuleNode>,
    stream: &model::Stream,
    nodes: &HashMap<String, RefCell<ModuleNode>>,
) -> Vec<errors::BuilderError> {
    let other = match nodes.get(&stream.to_node) {
        Some(n) => n,
        None => return vec![errors::BuilderError::NoSuchNode(errors::ErrorDetails {
            message: format!("Node not found: {}", stream.to_node),
            source: stream.source.clone(),
            related: vec![
                source.borrow().node.source.clone(),
            ],
        })],
    };
    let mut errs = Vec::new();
    let (mut source, mut target, source_stream_type, source_stream_decl, target_stream_type, target_stream_decl) = match stream.mode {
        model::StreamMode::Input => {
            let target = source.borrow_mut();
            let source = other.borrow_mut();
            let source_streams = match &source.module.stream_struct {
                Some(streams) => Some(streams),
                None => {
                    errs.push(errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                        message: format!("Module {} for node {} has no streams", source.module.name, source.node.name),
                        source: stream.source.clone(),
                        related: vec![source.node.source.clone()],
                    }));
                    None
                }
            };
            let target_streams = match &target.module.stream_struct {
                Some(streams) => Some(streams),
                None => {
                    errs.push(errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                        message: format!("Module {} for node {} has no streams", target.module.name, target.node.name),
                        source: stream.source.clone(),
                        related: vec![target.node.source.clone()],
                    }));
                    None
                }
            };
            if !errs.is_empty() {
                return errs;
            }
            // stream defines the "to" section as the source, because this is an input.
            let source_stream = match find_target_stream(&stream.source, &stream.to_stream, source_streams.unwrap()) {
                Ok(n) => Some(n),
                Err(e) => {
                    errs.extend(e);
                    None
                }
            };
            let target_stream = match find_source_stream(stream, target_streams.unwrap()) {
                Ok(n) => Some(n),
                Err(e) => {
                    errs.extend(e);
                    None
                }
            };
            if !errs.is_empty() {
                return errs;
            }

            let (src_type, src_decl) = source_stream.unwrap();
            let (tgt_type, tgt_decl) = target_stream.unwrap();
            (source, target, src_type, src_decl, tgt_type, tgt_decl)
        },
        model::StreamMode::Output => {
            let target = other.borrow_mut();
            let source = source.borrow_mut();
            let source_streams = match &source.module.stream_struct {
                Some(streams) => Some(streams),
                None => {
                    errs.push(errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                        message: format!("Module {} for node {} has no streams", source.module.name, source.node.name),
                        source: stream.source.clone(),
                        related: vec![source.node.source.clone()],
                    }));
                    None
                }
            };
            let target_streams = match &target.module.stream_struct {
                Some(streams) => Some(streams),
                None => {
                    errs.push(errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                        message: format!("Module {} for node {} has no streams", target.module.name, target.node.name),
                        source: stream.source.clone(),
                        related: vec![target.node.source.clone()],
                    }));
                    None
                }
            };
            if !errs.is_empty() {
                return errs;
            }
            let source_stream = match find_source_stream(&stream, source_streams.unwrap()) {
                Ok(n) => Some(n),
                Err(e) => {
                    errs.extend(e);
                    None
                }
            };
            // stream defines the "to" section as the target, because this is an output.
            let target_stream = match find_target_stream(&stream.source, &stream.to_stream, target_streams.unwrap()) {
                Ok(n) => Some(n),
                Err(e) => {
                    errs.extend(e);
                    None
                }
            };
            if !errs.is_empty() {
                return errs;
            }

            let (src_type, src_decl) = source_stream.unwrap();
            let (tgt_type, tgt_decl) = target_stream.unwrap();
            (source, target, src_type, src_decl, tgt_type, tgt_decl)
        },
    };

    if let meta::StreamType::Output(_) = source_stream_type {
        errs.push(errors::BuilderError::InvalidAst(errors::ErrorDetails {
            // TODO add more description on the module + node
            message: format!("Source stream {} is not an Input stream", stream.to_node),
            source: stream.source.clone(),
            related: vec![source.node.source.clone()],
        }));
    }
    if let meta::StreamType::Input(_) = target_stream_type {
        errs.push(errors::BuilderError::InvalidAst(errors::ErrorDetails {
            // TODO add more description on the module + node
            message: format!("Target stream {} is not an Output stream", stream.to_node),
            source: stream.source.clone(),
            related: vec![target.node.source.clone()],
        }));
    }
    if errs.is_empty() {
        let node_stream = Rc::new(NodeStream::new(
            source.node_id.clone(),
            target.node_id.clone(),
            source_stream_type,
            source_stream_decl,
            target_stream_type,
            target_stream_decl,
        ));
        target.add_dest_stream(node_stream.clone());
        source.add_src_stream(node_stream);
    }

    errs
}

/// Find the module stream referenced by the stream's source (fd and/or name).
fn find_source_stream(
    stream: &model::Stream,
    mod_streams: &meta::ModuleStreamStructure,
) -> Result<(meta::StreamType, meta::StreamDeclaration), Vec<errors::BuilderError>> {
    // Look at the source...
    for fixed in mod_streams.fixed_streams.iter() {
        if let Some(fd) = stream.fd {
            if let Some(s_fd) = fixed.fd_index {
                if s_fd == fd as usize {
                    return Ok((
                        fixed.stream_type.clone(),
                        meta::StreamDeclaration::FdIndex(s_fd as u16),
                    ));
                }
            }
        }
        if let Some(name) = &stream.name {
            if let Some(s_name) = &fixed.name {
                if *s_name == *name {
                    return Ok((
                        fixed.stream_type.clone(),
                        meta::StreamDeclaration::Name(s_name.clone()),
                    ));
                }
            }
        }
    }
    // Then check the dynamic streams.
    if let Some(name) = &stream.name {
        if let Some(input) = &mod_streams.input_variable {
            if input.field_name == *name {
                return Ok((
                    meta::StreamType::Input(input.stream_type.clone()),
                    meta::StreamDeclaration::VariableStream(name.clone(), meta::StreamDirection::Input),
                ));
            }
        }
        if let Some(output) = &mod_streams.output_variable {
            if output.field_name == *name {
                return Ok((
                    meta::StreamType::Output(output.stream_type.clone()),
                    meta::StreamDeclaration::VariableStream(name.clone(), meta::StreamDirection::Output),
                ));
            }
        }
    }
    Err(vec![errors::BuilderError::StreamNotFound(errors::ErrorDetails {
        message: format!("Stream {} not found in module streams", stream.name.as_deref().unwrap_or("unknown")),
        source: stream.source.clone(),
        related: vec![],
    })])
}

/// Find the module stream referenced by the target node's stream identifier.
fn find_target_stream(
    source: &model::Source,
    stream: &model::StreamToStream,
    mod_streams: &meta::ModuleStreamStructure,
) -> Result<(meta::StreamType, meta::StreamDeclaration), Vec<errors::BuilderError>> {
    match stream {
        model::StreamToStream::Integer(fd) => {
            // Try the fixed streams first.
            if let Some(s_fd) = mod_streams.fixed_streams.iter().find(|s| s.fd_index == Some(*fd as usize)) {
                return Ok((s_fd.stream_type.clone(), meta::StreamDeclaration::FdIndex(*fd as u16)));
            }
            // FD indexed streams can never reference variable streams.
            return Err(vec![errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                message: format!("Stream with fd index {} not found in module streams", fd),
                source: source.clone(),
                related: vec![],
            })]);
        }
        model::StreamToStream::String(name) => {
            // Try the fixed streams first.
            if let Some(s_name) = mod_streams.fixed_streams.iter().find(|s| s.name.as_ref() == Some(name)) {
                return Ok((s_name.stream_type.clone(), meta::StreamDeclaration::Name(name.clone())));
            }
            // Then the variable streams.
            if let Some(input) = &mod_streams.input_variable {
                if input.field_name == *name {
                    return Ok((
                        meta::StreamType::Input(input.stream_type.clone()),
                        meta::StreamDeclaration::VariableStream(name.clone(), meta::StreamDirection::Input),
                    ));
                }
            }
            if let Some(output) = &mod_streams.output_variable {
                if output.field_name == *name {
                    return Ok((
                        meta::StreamType::Output(output.stream_type.clone()),
                        meta::StreamDeclaration::VariableStream(name.clone(), meta::StreamDirection::Output),
                    ));
                }
            }

            return Err(vec![errors::BuilderError::StreamNotFound(errors::ErrorDetails {
                message: format!("Stream with name {} not found in module streams", name),
                source: source.clone(),
                related: vec![],
            })]);
        }
    }
}


#[derive(Clone, Debug)]
pub struct NodeStream {
    pub source_id: String,
    pub dest_id: String,
    pub source_type: meta::StreamType, // better be Output
    pub source_decl: meta::StreamDeclaration,
    pub dest_type: meta::StreamType, // better be Input
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
