use std::ffi::OsString;

use crate::server_shell::{
    builder::{self, errors},
    lls,
};

mod samples;
mod server_shell;
mod shell_lib;

fn main() {
    // TODO add real arg parsing.
    let action = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());
    if action == "help" {
        println!("Usage: llsio <action> LLS file>");
        println!("Actions:");
        println!("  validate - Validate the LLS");
        println!(
            "  build    - Build the script from the LLS.  Takes an extra argument, the output source directory (defaults to 'script-source')."
        );
        println!("  help     - Show this help message");
    } else if action == "validate" {
        match lls::llsio::read_file(
            std::path::PathBuf::from(
                std::env::args_os()
                    .nth(2)
                    .unwrap_or(OsString::from("lls.json")),
            )
            .as_path(),
        ) {
            Ok(ast) => {
                let errors = lls::validate::validate(&ast);
                if errors.is_empty() {
                    println!("LLS is valid.");
                } else {
                    for error in errors {
                        println!("{}", error);
                    }
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error loading AST: {}", e);
                std::process::exit(2);
            }
        }
    } else if action == "build" {
        match lls::llsio::read_file(
            std::path::PathBuf::from(
                std::env::args_os()
                    .nth(2)
                    .unwrap_or(OsString::from("lls.json")),
            )
            .as_path(),
        ) {
            Ok(ast) => {
                let errors = lls::validate::validate(&ast);
                if !errors.is_empty() {
                    for error in errors {
                        println!("{}", error);
                    }
                    std::process::exit(1);
                }
                let script_dir = std::env::args_os()
                    .nth(3)
                    .unwrap_or(OsString::from("script-source"));
                let write = match builder::writer::FileSourceWriter::new(
                    std::path::PathBuf::from(script_dir.clone()).as_path(),
                ) {
                    Ok(f) => f,
                    Err(e) => {
                        errors::report_errors(&e);
                        std::process::exit(3);
                    }
                };
                let issues = builder::from_lls::lls_to_module_source(&ast, write);
                if issues.has_issues() {
                    eprintln!("Error encountered with script");
                    builder::errors::report_errors(&issues.into());
                    std::process::exit(4);
                }
                println!("Module source written to {}", script_dir.to_string_lossy());
            }
            Err(e) => {
                eprintln!("Error loading LLS: {}", e);
                std::process::exit(2);
            }
        }
    } else if action == "cat-sample" {
        // Capture the command line arguments for the sample, skipping over the 'sample' action argument.
        let mut arg_iter = std::env::args().into_iter();
        let mut argv = vec![arg_iter.next().expect("No program name provided")];
        arg_iter.next();
        for arg in arg_iter {
            argv.push(arg);
        }
        crate::samples::cat_cp::main::main(argv, std::env::vars_os().collect());
    } else if action == "tee-sample" {
        // Capture the command line arguments for the sample, skipping over the 'sample' action argument.
        let mut arg_iter = std::env::args().into_iter();
        let mut argv = vec![arg_iter.next().expect("No program name provided")];
        arg_iter.next();
        for arg in arg_iter {
            argv.push(arg);
        }
        //crate::samples::tee_merge::main::main(argv, std::env::vars().collect());
        todo!();
    } else {
        eprintln!("Unknown action: {}", action);
    }
}
