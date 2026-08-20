//! A Rust code generator.
//! Allows for generating valid Rust syntax through a stateful interface.

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct CodeGen {
    files: HashMap<String, FileCode>,
}

pub struct FileCode {
    use_items: HashSet<String>,
    contents: Rc<BlockCode>,
}

pub struct BlockCode {
    parts: Vec<BlockPart>,
    terminates: bool,
}

pub enum BlockPart {
    Block(Rc<BlockCode>),
    Statement(String),
}
