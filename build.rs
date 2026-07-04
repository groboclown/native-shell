// SPDX-License-Identifier: MIT

use std::{collections, fs, path::Path};

use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpaceSettings};
use zip::{
    CompressionMethod, ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

fn main() {
    convert_yaml();
    build_script_t2();
    build_lls();
    build_shell_lib_zip();
}

fn convert_yaml() {
    build_json_schema("lls.schema.yaml");
    // Note: adding files here should then correspond to adding tests in the
    // server_shell/lls/llsio.rs file.
    build_json_from_yaml("src/samples/cat_cp/lls.yaml", ".json");
    build_json_from_yaml("src/samples/tee_merge/lls.yaml", ".json");
}

fn build_lls() {
    println!("cargo:rerun-if-changed=lls.schema.json");
    let content = std::fs::read_to_string("lls.schema.json").unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();
    let res = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string())
        .expect("Failed to format typespace");
    std::fs::write("src/server_shell/lls/model.rs", res).expect("Failed to write model file");
}

fn build_script_t2() {
    println!("cargo:rerun-if-changed=script-t2.schema.json");
    let content = std::fs::read_to_string("script-t2.schema.json").unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();
    let res = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string())
        .expect("Failed to format typespace");
    std::fs::write("src/server_shell/script2/model.rs", res).expect("Failed to write model file");
}

fn build_shell_lib_zip() {
    println!("cargo:rerun-if-changed=src/shell_lib");
    let zip_file = fs::File::create("shell_lib.zip").expect("Failed to create shell_lib.zip");
    let file_options: FileOptions<'_, _> = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let dir_options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);
    let mut writer = ZipWriter::new(zip_file);

    // Base files.
    writer
        .add_directory("src/", dir_options)
        .expect("Failed to add directory to zip");
    writer
        .start_file("src/shell_lib.rs", file_options)
        .expect("Failed to start zip file entry");
    let mut f = fs::File::open("src/shell_lib.rs").expect("Failed to open shell_lib file");
    std::io::copy(&mut f, &mut writer).expect("Failed to copy shell_lib file to zip");

    let mut dirstack = vec!["src/shell_lib".to_string()];
    while !dirstack.is_empty() {
        let zip_path = dirstack.pop().unwrap();
        let fs_path = Path::new(&zip_path);
        writer
            .add_directory(&zip_path, dir_options)
            .expect("Failed to add directory to zip");
        for entry in fs::read_dir(fs_path).expect("Failed to read shell_lib dir") {
            let entry = entry.expect("Failed to read shell_lib dir entry");
            let path = entry.path();
            let name = entry
                .file_name()
                .into_string()
                .expect("Failed to convert filename");
            let zip_entry_path = format!("{}/{}", &zip_path, name);
            if path.is_file() {
                writer
                    .start_file(zip_entry_path, file_options)
                    .expect("Failed to start zip file entry");
                let mut f = fs::File::open(&path).expect("Failed to open shell_lib file");
                std::io::copy(&mut f, &mut writer).expect("Failed to copy shell_lib file to zip");
            } else if path.is_dir() {
                dirstack.push(zip_entry_path.clone());
            }
        }
    }

    writer
        .finish()
        .expect("Failed to finish writing shell_lib.zip");
}

fn build_json_from_yaml(path: &str, ext: &str) {
    println!("cargo:rerun-if-changed={}", path);
    let out_path = path.replace(".yaml", ext);
    println!("cargo:rerun-if-changed={}", out_path);
    let content = fs::read_to_string(path).expect("Failed to read YAML file");
    let _ = fs::remove_file(&out_path);
    let mut output = fs::File::create(out_path.clone())
        .expect(format!("Failed to open output file {}", out_path).as_str());

    let converter = yaml2json_rs::Yaml2Json::new(yaml2json_rs::Style::PRETTY);
    converter
        .document_to_writer(&content, &mut output)
        .expect("Failed to convert YAML to JSON");
}

/// Expand the lls.schema.yaml into a JSON schema, proper for
/// typify to turn into Rust syntax.
fn build_json_schema(path: &str) {
    build_json_from_yaml(path, ".pre-json");
    let pre_path = path.replace(".yaml", ".pre-json");
    let out_path = path.replace(".yaml", ".json");

    let root: serde_json::Value = serde_json::from_str(
        fs::read_to_string(&pre_path)
            .expect("Failed to read JSON file")
            .as_str(),
    )
    .expect("Failed to handle JSON file");

    // For each [ ... { "$ref": "#/$composite/..." } ] entry,
    // replace with the corresponding composite entry.
    let (root, composites) = get_composites(root);
    let root = replace_composite_refs(&root, &composites);

    // Finally, output the assembled file.
    let _ = fs::remove_file(&pre_path);
    let _ = fs::remove_file(&out_path);
    // ... use pretty formatting.
    let contents = format!("{:#}", root).replace("#/$composite/", "#/$defs/");
    fs::write(out_path, contents).expect("Failed to write output file");
}

/// Recursively replace composite references.
fn replace_composite_refs(
    root: &serde_json::Value,
    composites: &collections::HashMap<String, Vec<serde_json::Value>>,
) -> serde_json::Value {
    match root {
        serde_json::Value::Array(values) => {
            let mut ret: Vec<serde_json::Value> = Vec::new();
            for v in values {
                if let serde_json::Value::Object(map) = v {
                    if let Some(r1) = map.get("$ref") {
                        if let serde_json::Value::String(r2) = r1 {
                            if let Some(r3) = composites.get(r2) {
                                ret.append(&mut (r3.clone()))
                            } else {
                                // normal $ref
                                ret.push(v.clone())
                            }
                        } else {
                            // weird $ref -> non string
                            ret.push(v.clone())
                        }
                    } else {
                        ret.push(replace_composite_refs(v, composites));
                    }
                } else {
                    ret.push(replace_composite_refs(v, composites));
                }
            }
            serde_json::Value::Array(ret)
        }
        serde_json::Value::Object(map) => {
            let mut ret = serde_json::Map::new();
            for entry in map.iter() {
                ret.insert(entry.0.clone(), replace_composite_refs(entry.1, composites));
            }
            serde_json::Value::Object(ret)
        }
        _ => root.clone(),
    }
}

/// Extract the '$composites' root entry.
/// This must be in the form:
/// $composites:
///   name:
///     oneOf:
///       - value
fn get_composites(
    mut root: serde_json::Value,
) -> (
    serde_json::Value,
    collections::HashMap<String, Vec<serde_json::Value>>,
) {
    let mut ret = collections::HashMap::new();
    if let serde_json::Value::Object(mut map) = root {
        let mut defs = serde_json::Map::new();
        if let Some(m) = map.remove("$defs") {
            if let serde_json::Value::Object(m) = m {
                defs = m;
            }
        }
        if let Some(m) = map.remove("$composite") {
            if let serde_json::Value::Object(map) = m {
                defs.append(&mut map.clone());
                for entry in map.iter() {
                    let key = entry.0.clone();
                    if let serde_json::Value::Object(entry_m) = &entry.1 {
                        if let Some(m) = entry_m.get("oneOf") {
                            if let serde_json::Value::Array(a) = m {
                                ret.insert(format!("#/$composite/{}", key), a.clone());
                            }
                        }
                    }
                }
            }
        }
        map.insert("$defs".into(), serde_json::Value::Object(defs));
        root = serde_json::Value::Object(map);
    }
    (root, ret)
}
