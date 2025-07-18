use crate::server_shell::{ast, builder::from_ast::ast_to_module_source};

mod shell_lib;
mod server_shell;
mod samples;


fn main() {
    // TODO add real arg parsing.
    let action = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());
    if action == "help" {
        println!("Usage: astio <action> <ast file>");
        println!("Actions:");
        println!("  validate - Validate the AST");
        println!("  build    - Build the script from the AST");
        println!("  help     - Show this help message");
    } else if action == "validate" {
        match ast::astio::read_file(std::env::args().nth(2).unwrap_or_else(|| "ast.json".to_string())) {
            Ok(ast) => {
                let errors = ast::validate::validate(&ast);
                if errors.is_empty() {
                    println!("AST is valid.");
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
        match ast::astio::read_file(std::env::args().nth(2).unwrap_or_else(|| "ast.json".to_string())) {
            Ok(ast) => {
                let errors = ast::validate::validate(&ast);
                if !errors.is_empty() {
                    for error in errors {
                        println!("{}", error);
                    }
                    std::process::exit(1);
                }
                let module_file = std::env::args().nth(3).unwrap_or_else(|| "module.rs".to_string());
                let file = match std::fs::File::create(&module_file) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating file {}: {}", module_file, e);
                        std::process::exit(3);
                    }
                };
                match ast_to_module_source(&ast, file) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error writing module source: {}", e);
                        std::process::exit(4);
                    }
                }
                println!("Module source written to {}", module_file);
            }
            Err(e) => {
                eprintln!("Error loading AST: {}", e);
                std::process::exit(2);
            }
        }
    } else if action == "sample" {
        // Capture the command line arguments for the sample, skipping over the 'sample' action argument.
        let mut arg_itr = std::env::args().into_iter();
        let mut argv = vec![arg_itr.next().expect("No program name provided")];
        arg_itr.next();
        for arg in arg_itr {
            argv.push(arg);
        }
        crate::samples::cat_cp::main::main(argv, std::env::vars().collect());
    } else {
        eprintln!("Unknown action: {}", action);
    }
}
