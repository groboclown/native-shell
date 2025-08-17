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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cat_cp_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/cat_cp/ast.json")).expect("failed to utf8 convert json");
        let ast = read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation errors: {:?}", ast_errors);
    }
    
    #[test]
    fn test_tee_merge_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/tee_merge/ast.json")).expect("failed to utf8 convert json");
        let ast = read_str(&json.to_string()).expect("failed to read json");
        let ast_errors = crate::server_shell::ast::validate::validate(&ast);
        assert!(ast_errors.is_empty(), "AST validation errors: {:?}", ast_errors);
    }
}
