//! Helper for the main program to exit and handle reporting errors.


/// Handle the exit of the main program, with the correct exit code and error reporting.
pub fn handle_exit(res: Result<i32, String>) {
    match res {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
