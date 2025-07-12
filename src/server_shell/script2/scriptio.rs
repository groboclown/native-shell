use super::model;


/// Read the model as a string value.
pub fn read_str(data: String) -> Result<model::Script2, serde_json::Error> {
    serde_json::from_str(&data)
}

/// Read the model from a reader.
pub fn read_reader<R: std::io::Read>(r: R) -> Result<model::Script2, serde_json::Error> {
    serde_json::from_reader(r)
}

pub fn read_file(path: String) -> Result<model::Script2, serde_json::Error> {
    let file = std::fs::File::open(path).map_err(|e| serde_json::Error::io(e))?;
    read_reader(file)
}
