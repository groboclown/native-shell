//! Helpers and tools for managing Cargo library declarations.

use crate::{server_shell::builder::errors::BuilderError, shell_lib::compile::meta::{CrateDependency, VerBit}};
use super::crates_io;

/// Fetches crate information from crates.io and constructs a RustDependency.
pub fn get_crate_dependency<'a>(crate_name: &'a str) -> Result<CrateDependency, BuilderError> {
    let crate_info = crates_io::read_crate_info(crate_name).map_err(from_ureq_err)?;
    let version_str = crate_info.crate_data.default_version;
    let version_bits = parse_version_str(&version_str);
    Ok(CrateDependency {
        name: crate_name.to_string(),
        version: version_bits,
        features: vec![],
    })
}

/// Constructs a RustDependency from a crate name and version string.
pub fn as_crate_dependency<'a>(crate_name: &'a str, version: &'a str) -> CrateDependency {
    let version_bits = parse_version_str(version);
    CrateDependency {
        name: crate_name.to_string(),
        version: version_bits,
        features: vec![],
    }
}

pub fn evaluate_crate_dependency(cr: &CrateDependency) -> Result<CrateDependency, BuilderError> {
    if cr.version.len() > 0 {
        return Ok(cr.clone());
    }
    get_crate_dependency(&cr.name)
}

fn parse_version_str<'a>(version: &'a str) -> Vec<VerBit> {
    let mut bits = Vec::new();
    let mut buff = String::new();
    let mut n_buff = 0;
    let mut sep = '\0';
    let mut mode = 0; // 0 == unknown, 1 = number, 2 = string
    for c in version.chars() {
        match c {
            '0'..='9' => {
                match mode {
                    0 => {
                        mode = 1;
                        n_buff = 0;
                    }
                    // 1 => continue number part
                    2 => {
                        // Flush string part.
                        bits.push(VerBit::S((sep, buff.clone())));
                        mode = 1;
                        sep = '\0';
                        n_buff = 0;
                    }
                    _ => {}
                }
                n_buff = n_buff * 10 + (c as u8 - b'0') as u32;
            }
            '.' | '-' | '_' | '+' => {
                match mode {
                    0 => {
                        // Two separators in a row.  Count as an empty string part.
                        bits.push(VerBit::S((sep, String::new())));
                    }
                    1 => {
                        // Flush number part.
                        bits.push(VerBit::N((sep, n_buff)));
                    }
                    2 => {
                        // Flush string part.
                        bits.push(VerBit::S((sep, buff.clone())));
                    }
                    _ => {}
                }
                mode = 0;
                sep = c;
            }
            _ => {
                match mode {
                    0 => {
                        mode = 2;
                        buff.clear();
                    }
                    1 => {
                        // Flush number part.
                        bits.push(VerBit::N((sep, n_buff)));
                        mode = 2;
                        sep = '\0';
                        buff.clear();
                    }
                    // 2 => continue string part
                    _ => {}
                }
                buff.push(c);
            }
        }
    }
    match mode {
        1 => {
            // Flush number part.
            bits.push(VerBit::N((sep, n_buff)));
        }
        2 => {
            // Flush string part.
            bits.push(VerBit::S((sep, buff.clone())));
        }
        _ => {
            // Possibly read a trailing separator.
            if sep != '\0' {
                bits.push(VerBit::S((sep, String::new())));
            }
        }
    }
    bits
}

fn from_ureq_err(err: ureq::Error) -> BuilderError {
    match err {
        ureq::Error::StatusCode(code) => {
            BuilderError::HttpError(format!("Crates.io returned status code {}", code))
        }
        ureq::Error::HostNotFound => {
            BuilderError::HttpError("Could not find crates.io site".to_string())
        }
        ureq::Error::Timeout(_) => {
            BuilderError::HttpError("Timeout while connecting to crates.io".to_string())
        }
        ureq::Error::Io(err) => {
            BuilderError::HttpError(format!("I/O error while connecting to crates.io: {}", err))
        }
        err => {
            BuilderError::HttpError(format!("error while connecting to crates.io: {}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_version_str;
    use crate::shell_lib::compile::meta::VerBit;

    fn repr(bits: Vec<VerBit>) -> Vec<(char, Option<u32>, Option<String>)> {
        bits.into_iter()
            .map(|b| match b {
                VerBit::N((sep, n)) => (sep, Some(n), None),
                VerBit::S((sep, s)) => (sep, None, Some(s)),
            })
            .collect()
    }

    #[test]
    fn parses_simple_semver() {
        let bits = parse_version_str("1.2.3");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
            ]
        );
    }

    #[test]
    fn parses_prerelease_with_dash() {
        let bits = parse_version_str("1.0.0-alpha");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', Some(0), None),
                ('.', Some(0), None),
                ('-', None, Some("alpha".to_string())),
            ]
        );
    }

    #[test]
    fn parses_leading_v_string() {
        let bits = parse_version_str("v1.2.3");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', None, Some("v".to_string())),
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
            ]
        );
    }

    #[test]
    fn parses_build_metadata_plus() {
        let bits = parse_version_str("1.2.3+build.1");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
                ('+', None, Some("build".to_string())),
                ('.', Some(1), None),
            ]
        );
    }

    #[test]
    fn parses_prerelease_with_dot_number() {
        let bits = parse_version_str("1.2.3-alpha.1");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
                ('-', None, Some("alpha".to_string())),
                ('.', Some(1), None),
            ]
        );
    }

    #[test]
    fn parses_date_like_version() {
        let bits = parse_version_str("2020.10.31");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(2020), None),
                ('.', Some(10), None),
                ('.', Some(31), None),
            ]
        );
    }

    #[test]
    fn parses_single_number() {
        let bits = parse_version_str("1");
        assert_eq!(repr(bits), vec![('\0', Some(1), None)]);
    }

    #[test]
    fn parses_empty_part_between_separators() {
        let bits = parse_version_str("1..2");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', None, Some(String::new())),
                ('.', Some(2), None),
            ]
        );
    }

    #[test]
    fn parses_rc_with_underscore_and_dash() {
        let bits = parse_version_str("1-rc1");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('-', None, Some("rc".to_string())),
                ('\0', Some(1), None),
            ]
        );
        let bits2 = parse_version_str("1_2_3");
        assert_eq!(
            repr(bits2),
            vec![
                ('\0', Some(1), None),
                ('_', Some(2), None),
                ('_', Some(3), None),
            ]
        );
    }

    #[test]
    fn parses_trailing_separator_dot_and_dash() {
        let bits = parse_version_str("1.2.3.");
        assert_eq!(
            repr(bits),
            vec![
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
                ('.', None, Some(String::new())),
            ]
        );

        let bits2 = parse_version_str("1.2.3-");
        assert_eq!(
            repr(bits2),
            vec![
                ('\0', Some(1), None),
                ('.', Some(2), None),
                ('.', Some(3), None),
                ('-', None, Some(String::new())),
            ]
        );
    }
}
