use crate::server_shell::ast;

mod shell_lib;
mod server_shell;


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
                }
            }
            Err(e) => eprintln!("Error loading AST: {}", e),
        }
    } else if action == "build" {
        match ast::astio::read_file(std::env::args().nth(2).unwrap_or_else(|| "ast.json".to_string())) {
            Ok(ast) => {
                let errors = ast::validate::validate(&ast);
                if errors.is_empty() {
                    println!("AST is valid.");
                } else {
                    for error in errors {
                        println!("{}", error);
                    }
                }
            }
            Err(e) => eprintln!("Error loading AST: {}", e),
        }
    } else {
        eprintln!("Unknown action: {}", action);
    }
}
