//! Abstraction and implementation of writing sources.

pub trait SourceWriter {
    fn writer_for(&self, file_name: &str) -> Box<dyn std::io::Write>;
}


pub struct FileSourceWriter {
    pub base_path: String,
}

impl SourceWriter for FileSourceWriter {
    fn writer_for(&self, file_name: &str) -> Box<dyn std::io::Write> {
        let full_path = format!("{}/{}", self.base_path, file_name);
        let file = std::fs::File::create(full_path).expect("Failed to create file");
        Box::new(file)
    }
}

impl FileSourceWriter {
    pub fn new(base_path: &str) -> Result<Self, std::io::Error> {
        std::fs::create_dir_all(base_path)?;
        Ok(FileSourceWriter {
            base_path: base_path.to_string(),
        })
    }
}
