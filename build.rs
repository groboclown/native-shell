// SPDX-License-Identifier: MIT

use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    build_script_t2();
    build_ast();
}

fn build_ast() {
    println!("cargo:rerun-if-changed=ast.schema.json");
    let content = std::fs::read_to_string("ast.schema.json").unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();    
    let res = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string()).expect("Failed to format typespace");
    std::fs::write("src/server_shell/ast/model.rs", res).expect("Failed to write model file");
}

fn build_script_t2() {
    println!("cargo:rerun-if-changed=script-t2.schema.json");
    let content = std::fs::read_to_string("script-t2.schema.json").unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();    
    let res = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string()).expect("Failed to format typespace");
    std::fs::write("src/server_shell/script2/model.rs", res).expect("Failed to write model file");
}
