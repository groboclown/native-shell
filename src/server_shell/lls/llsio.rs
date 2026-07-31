use super::model;

/// Read the model as a string value.
pub fn read_string_ref(
    data: &String,
) -> Result<model::NativeShellLowLevelScriptSchema, serde_json::Error> {
    serde_json::from_str(data)
}

/// Read the model from a reader.
pub fn read_reader<R: std::io::Read>(
    r: R,
) -> Result<model::NativeShellLowLevelScriptSchema, serde_json::Error> {
    serde_json::from_reader(r)
}

/// Read the model from a file path.
pub fn read_file<'a, P: Into<&'a std::path::Path>>(
    path: P,
) -> Result<model::NativeShellLowLevelScriptSchema, serde_json::Error> {
    let file = std::fs::File::open(path.into()).map_err(|e| serde_json::Error::io(e))?;
    read_reader(file)
}

/// Write the model to a file.
pub fn write_file(
    path: String,
    lls: &model::NativeShellLowLevelScriptSchema,
) -> Result<(), serde_json::Error> {
    let file = std::fs::File::create(path).map_err(|e| serde_json::Error::io(e))?;
    serde_json::to_writer(file, lls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_cp_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/cat_cp/lls.json"))
            .expect("failed to utf8 convert json");
        let lls = read_string_ref(&json.to_string()).expect("failed to read json");
        let lls_errors = crate::server_shell::lls::validate::validate(&lls);
        assert!(
            lls_errors.is_empty(),
            "LLS validation errors: {:?}",
            lls_errors
        );
    }

    #[test]
    fn test_tee_merge_json() {
        let json = std::str::from_utf8(include_bytes!("../../samples/tee_merge/lls.json"))
            .expect("failed to utf8 convert json");
        let lls = read_string_ref(&json.to_string()).expect("failed to read json");
        let lls_errors = crate::server_shell::lls::validate::validate(&lls);
        assert!(
            lls_errors.is_empty(),
            "LLS validation errors: {:?}",
            lls_errors
        );
    }
}
