// SPDX-License-Identifier: MIT

use std::{fs::{self, File}, path::Path};

use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpaceSettings};
use zip::{write::{FileOptions, SimpleFileOptions}, CompressionMethod, ZipWriter};

fn main() {
    build_script_t2();
    build_ast();
    build_shell_lib_zip();
    build_sample_json();
}

fn build_ast() {
    println!("cargo:rerun-if-changed=ast.schema.json");
    // Note: While I think yaml is much easier to read than json, many IDE tools have support for json schema
    //   stored in json, not yaml.  Switching this to yaml is very simple from the build.  It's just changing the
    //   if-changed to a yaml, and run the conversion function.
    // build_json_from_yaml("ast.schema.yaml");
    let content = std::fs::read_to_string("ast.schema.json").unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();
    let res = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string()).expect("Failed to format typespace");
    std::fs::write("src/server_shell/ast/model.rs", res).expect("Failed to write model file");
}

fn build_sample_json() {
    build_json_from_yaml("src/samples/cat_cp/ast.yaml");
    build_json_from_yaml("src/samples/tee_merge/ast.yaml");
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

fn build_shell_lib_zip() {
    println!("cargo:rerun-if-changed=src/shell_lib");
    let zip_file = File::create("shell_lib.zip").expect("Failed to create shell_lib.zip");
    let file_options: FileOptions<'_, _> = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let dir_options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);
    let mut writer = ZipWriter::new(zip_file);

    // Base files.
    writer.add_directory("src/", dir_options).expect("Failed to add directory to zip");
    writer.start_file("src/shell_lib.rs", file_options).expect("Failed to start zip file entry");
    let mut f = File::open("src/shell_lib.rs").expect("Failed to open shell_lib file");
    std::io::copy(&mut f, &mut writer).expect("Failed to copy shell_lib file to zip");

    let mut dirstack = vec!["src/shell_lib".to_string()];
    while !dirstack.is_empty() {
        let zip_path = dirstack.pop().unwrap();
        let fs_path = Path::new(&zip_path);
        writer.add_directory(&zip_path, dir_options).expect("Failed to add directory to zip");
        for entry in fs::read_dir(fs_path).expect("Failed to read shell_lib dir") {
            let entry = entry.expect("Failed to read shell_lib dir entry");
            let path = entry.path();
            let name = entry.file_name().into_string().expect("Failed to convert filename");
            let zip_entry_path = format!("{}/{}", &zip_path, name);
            if path.is_file() {
                writer.start_file(zip_entry_path, file_options).expect("Failed to start zip file entry");
                let mut f = File::open(&path).expect("Failed to open shell_lib file");
                std::io::copy(&mut f, &mut writer).expect("Failed to copy shell_lib file to zip");
            } else if path.is_dir() {
                dirstack.push(zip_entry_path.clone());
            }
        }
    }

    writer.finish().expect("Failed to finish writing shell_lib.zip");
}

fn build_json_from_yaml(path: &str) {
    println!("cargo:rerun-if-changed={}", path);
    let out_path = path.replace(".yaml", ".json");
    println!("cargo:rerun-if-changed={}", out_path);
    let content = std::fs::read_to_string(path).expect("Failed to read YAML file");
    let _ = fs::remove_file(&out_path);
    let mut output = fs::File::create(out_path.clone())
                .expect(format!("Failed to open output file {}", out_path).as_str());

    let converter = yaml2json_rs::Yaml2Json::new(yaml2json_rs::Style::PRETTY);
    converter
        .document_to_writer(&content, &mut output)
        .expect("Failed to convert YAML to JSON");
}
