use super::model;


/// Read the model as a string value.
pub fn read_str(data: &String) -> Result<model::NativeShellAstSchema, serde_json::Error> {
    serde_json::from_str(data)
}

/// Read the model from a reader.
pub fn read_reader<R: std::io::Read>(r: R) -> Result<model::NativeShellAstSchema, serde_json::Error> {
    serde_json::from_reader(r)
}

/// Read the model from a file path.
pub fn read_file(path: String) -> Result<model::NativeShellAstSchema, serde_json::Error> {
    let file = std::fs::File::open(path).map_err(|e| serde_json::Error::io(e))?;
    read_reader(file)
}

/// Write the model to a file.
pub fn write_file(path: String, ast: &model::NativeShellAstSchema) -> Result<(), serde_json::Error> {
    let file = std::fs::File::create(path).map_err(|e| serde_json::Error::io(e))?;
    serde_json::to_writer(file, ast)
}
