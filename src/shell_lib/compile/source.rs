//! Records of the source location, to aid in runtime debugging.

use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Source {
    /// Source location file
    pub file: String,

    /// Source location line
    pub line: i64,

    /// Source location column
    pub column: i64,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.file)?;
        if self.line > 0 {
            f.write_str("@")?;
            f.write_fmt(format_args!("{}", self.line))?;
            if self.column > 0 {
                f.write_str(":")?;
                f.write_fmt(format_args!("{}", self.column))?;
            }
        }
        Ok(())
    }
}

impl Source {
    pub fn new(file: &str, line: i64, column: i64) -> Self {
        Self { file: file.to_string(), line, column }
    }
}

impl Default for Source {
    fn default() -> Self {
        Self {
            file: String::new(),
            line: 0,
            column: 0,
        }
    }
}
