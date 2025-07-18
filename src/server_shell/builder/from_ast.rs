//! Turns the AST into a module source.

use crate::server_shell::{ast::model::{NativeShellAstSchema, Node}};

pub fn ast_to_module_source<W: std::io::Write>(
    ast: &NativeShellAstSchema,
    out: W,
) -> Result<(), std::io::Error> {
    // out.write(join_uses(ast))?;
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ast_to_module_source not implemented",
    ));
}

/// Create the module's 'use' lines.
pub fn join_uses(ast: &NativeShellAstSchema) -> String {
    "not implemented".to_string()
    /*
    let mut ret = HashSet::new();
    for part in ast.nodes.iter() {
        if part.is_empty() {
            continue;
        }
        let mut joined = "use ".to_string();
        joined.push_str(part.join("::").as_str());
        joined.push_str(";\n");
        ret.insert(joined);
    }
    return ret.into_iter()
        .collect::<Vec<String>>()
        .join("");
    */
}
