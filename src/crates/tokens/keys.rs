/* The identifiers contains the standard rules for the language
Example:
 - Valid characters
 - Rasonable length
 - Case sensitivity
 - Reserved words
*/

use crate::crates::aux;

use std::collections::HashMap;
use std::string::String;

pub struct Processed<T>(T);

pub fn process<T: AsRef<str>>(value: T) -> Processed<String> {
    let processed_value = remove_quotes(value.as_ref());

    Processed(processed_value)
}
// Constants
const MAX_IDENTIFIER_LENGTH: usize = 255;
///! The function find the control flow identifiers
/// The function receives a string with the control flow identifier
/// received a string from the AST tree and returns the equivalent keyword in Rust
pub fn control_structure_to_rust(typescript_statement: Processed<String>) -> String {
    match typescript_statement.0.as_str() {
        "IfStatement" => "if".to_string(),
        "BlockStatement" => "else".to_string(),
        "ForStatement" => "for".to_string(),
        "ForInStatement" => "for {} in".to_string(),
        "WhileStatement" => "while".to_string(),
        "DoWhileStatement" => "loop".to_string(),
        "SwitchStatement" => "match".to_string(),
        "ContinueStatement" => "continue".to_string(),
        "BreakStatement" => "break".to_string(),
        "ReturnStatement" => "return".to_string(),
        "FunctionDeclaration" => "fn".to_string(),
        _ => "No control structure found".to_string(),
    }
}
///! The function find the variable identifiers
/// The function receives a string with the variable identifier
/// received a string from the AST tree and returns the equivalent keyword in Rust
fn variable_to_rust(str: &str, count: i32) -> &str {
    match str {
        "let" => {
            if count > 1 || count == 0 {
                "let mut"
            } else {
                "let"
            }
        }
        "var" => "let mut",
        "const" => "const",
        _ => "hello", // Para manejar casos no especificados
    }
}
///! The function find the data type identifiers
/// The function receives a string with the data type identifier
/// received a string from the AST tree and returns the equivalent keyword in Rust
pub fn data_types_to_rust(typescript_type: &str) -> &str {
    match typescript_type {
        // Números
        "Number" => "Number",
        "String" => "String",
        "Boolean" => "Boolean",

        // Tipos especiales
        "any" | "unknown" => "Any", // Requiere el trait `Any` de std
        "void" => "()",
        "never" => "!",
        "null" | "undefined" => "Option<()>",

        // Colecciones
        "Array" => "Vec<T>",
        "Set" => "HashSet<T>",
        "Map" => "HashMap<K, V>",

        // Tipos que no tienen equivalente directo
        "TSTypeAnnotation" => "enum", // Los enums en Rust son más poderosos
        "TSTupleType" => "(T, U, ...)", // Los tuples en Rust se escriben (T, U, ...)
        "ClassDeclaration" => "",
        "TSEnumDeclaration" => "",
        // Para tipos no reconocidos, devolvemos el tipo original
        _ => "",
    }
}
///! This function validates the identifier
/// The function receives a string with the identifier
/// then test if the identifier is not empty, starts with a letter, and contains only alphanumeric characters and underscores
fn validate_identifier(identifier: &str) -> Result<(), &'static str> {
    if identifier.is_empty() {
        return Err("Identifier cannot be empty");
    }
    if !identifier.chars().next().unwrap().is_alphabetic() {
        println!("This is the identifier: {}", identifier.to_string());
        return Err("Identifier must start with a letter");
    }
    if !identifier.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Identifier can only contain alphanumeric characters and underscores");
    }
    Ok(())
}
///! This function validates the identifier length
fn is_valid_identifier_length(identifier: &str) -> bool {
    identifier.len() <= MAX_IDENTIFIER_LENGTH
}

///! This function removes quotes from a string
fn remove_quotes(input: &str) -> String {
    input.chars().filter(|&c| c != '"' && c != '\'').collect()
}

///! This function generates a safe Rust name from an identifier
pub fn identifiers_to_rust(
    identifier: &str,
    data: &mut HashMap<String, i16>,
) -> Result<String, &'static str> {
    let clear_identifier = remove_quotes(identifier);
    validate_identifier(&clear_identifier)?;
    if !is_valid_identifier_length(&clear_identifier) {
        println!("{}", identifier);
        return Err("Identifier exceeds maximum length");
    }
    Ok(aux::safe_name::generate_safe_name(identifier, data))
}
