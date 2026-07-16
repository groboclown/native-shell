//SPDX:MIT

//! Functions that generate Rust-specific names for elements.
//! This allows for the generators to share this logic to ensure that they
//! refer to the same things.

use crate::shell_lib::structure::JobRef;

/// Create the command module name.
/// Usable for both the filename creation and the 'use mod' line.
pub fn command_module(job: JobRef) -> String {
    format!("c{}", job)
}

/// Create the name of the command's argument structure.
pub fn command_arg_struct(job: JobRef) -> String {
    format!("Cmd{}Args", job)
}

/// Create the name of the command's state structure.
pub fn command_state_struct(job: JobRef) -> String {
    format!("Cmd{}State", job)
}

/// Create the name of the command's CommandSetup + CommandHandler implementing structure.
pub fn command_struct(job: JobRef) -> String {
    format!("Cmd{}Runner", job)
}

/// Construct a unique, rust-compatible name from the string.
/// While not easy to read, it's guaranteed to be 1-to-1 unique with the
/// input name.
pub(crate) fn unique_name(name: &String) -> String {
    let mut ret = String::new();
    for c in name.chars() {
        // In order to keep this alphanumeric + unique,
        // replace these names with a special encoding.
        if c.is_ascii_alphanumeric() {
            ret.push(c);
        } else if c == '_' {
            ret.push('_');
            ret.push('_');
        } else {
            ret.push('_');
            let c: u32 = c.into();
            ret.push_str(format!("{:x}", c).as_str());
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_name() {
        let test_data = [
            ("", ""),
            ("a", "a"),
            ("B", "B"),
            ("a_b", "a__b"),
            ("a b", "a_20b"),
            ("%a $6", "_25a_20_246"),
        ];
        for (inp, exp) in test_data {
            assert_eq!(exp, unique_name(&inp.into()));
        }
    }
}
