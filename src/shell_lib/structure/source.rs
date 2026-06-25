//! Records of the source location, to aid in runtime debugging.

use std::fmt::Display;

/// A location in the source script, for debugging and detailed logging.
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
        self.file.fmt(f)?;
        if self.line > 0 {
            "@".fmt(f)?;
            self.line.fmt(f)?;
            if self.column > 0 {
                ":".fmt(f)?;
                self.column.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl Source {
    pub fn new(file: &str, line: i64, column: i64) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
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

/// A script resource, for debugging or detailed logging.
#[derive(Clone, Debug)]
pub struct Resource {
    /// Name of the resource.
    pub name: String,

    /// Resource kind.
    pub kind: String,

    /// Location in the source.
    pub source: Source,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({}) in ", self.name, self.kind))?;
        self.source.fmt(f)
    }
}

impl Resource {
    pub fn new(name: &str, kind: &str, file: &str, line: i64, column: i64) -> Self {
        Self {
            name: name.to_string(),
            kind: kind.to_string(),
            source: Source::new(file, line, column),
        }
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            name: String::new(),
            kind: String::new(),
            source: Source::default(),
        }
    }
}
