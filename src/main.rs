use crate::server_shell::{builder, lls};

mod samples;
mod server_shell;
mod shell_lib;

fn main() {
    // TODO add real arg parsing.
    let action = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());
    if action == "help" {
        println!("Usage: astio <action> <ast file>");
        println!("Actions:");
        println!("  validate - Validate the AST");
        println!(
            "  build    - Build the script from the AST.  Takes an extra argument, the output source directory (defaults to 'script-source')."
        );
        println!("  help     - Show this help message");
    } else if action == "validate" {
        match lls::llsio::read_file(
            std::env::args()
                .nth(2)
                .unwrap_or_else(|| "lls.json".to_string()),
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
            std::env::args()
                .nth(2)
                .unwrap_or_else(|| "lls.json".to_string()),
        ) {
            Ok(ast) => {
                let errors = lls::validate::validate(&ast);
                if !errors.is_empty() {
                    for error in errors {
                        println!("{}", error);
                    }
                    std::process::exit(1);
                }
                let script_dir = std::env::args()
                    .nth(3)
                    .unwrap_or_else(|| "script-source".to_string());
                let write = match builder::writer::FileSourceWriter::new(&script_dir) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating file {}: {}", script_dir, e);
                        std::process::exit(3);
                    }
                };
                let issues = builder::from_lls::lls_to_module_source(&ast, write);
                if issues.has_issues() {
                    eprintln!("Error encountered with script");
                    builder::errors::report_errors(&issues.into());
                    std::process::exit(4);
                }
                println!("Module source written to {}", script_dir);
            }
            Err(e) => {
                eprintln!("Error loading AST: {}", e);
                std::process::exit(2);
            }
        }
    } else if action == "cat-sample" {
        // Capture the command line arguments for the sample, skipping over the 'sample' action argument.
        let mut arg_itr = std::env::args().into_iter();
        let mut argv = vec![arg_itr.next().expect("No program name provided")];
        arg_itr.next();
        for arg in arg_itr {
            argv.push(arg);
        }
        crate::samples::cat_cp::main::main(argv, std::env::vars_os().collect());
    } else if action == "tee-sample" {
        // Capture the command line arguments for the sample, skipping over the 'sample' action argument.
        let mut arg_itr = std::env::args().into_iter();
        let mut argv = vec![arg_itr.next().expect("No program name provided")];
        arg_itr.next();
        for arg in arg_itr {
            argv.push(arg);
        }
        //crate::samples::tee_merge::main::main(argv, std::env::vars().collect());
        todo!();
    } else {
        eprintln!("Unknown action: {}", action);
    }
}
