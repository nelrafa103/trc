use std::collections::HashMap;
use std::collections::HashSet;
// Documentacion:
///! This module contains the limits for the parser.
///! This function test if a word is a Rust keyword.
///! # Arguments:
///! * `word` - The word to test.
///! # Returns:
///! * `bool` - True if the word is a Rust keyword, false otherwise.
fn is_rust_keyword(word: &str) -> bool {
    let keywords: HashSet<&str> = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ]
    .iter()
    .cloned()
    .collect();

    keywords.contains(word)
}
///! This function generates a safe name for a variable.
/// It takes a word and a hashmap of existing names.
/// If the word is a rust keyword, it prepends "safe_" to the word.
/// If the word is not a rust keyword, it replaces spaces with underscores.
/// If the word already exists in the hashmap, it appends a number to the word.
/// If the word does not exist in the hashmap, it adds the word to the hashmap.
/// The function returns the safe name.
pub fn generate_safe_name(word: &str, existing_names: &mut HashMap<String, i16>) -> String {
    if is_rust_keyword(word) {
        let mut safe_word = String::from("safe_");
        safe_word.push_str(&word.to_lowercase());
        if existing_names.contains_key(word) == false {
            existing_names.insert(word.to_string(), 1);
            safe_word.push_str("_1");
        } else {
            if let Some(value) = existing_names.get_mut(word) {
                *value += 1;
            }
            safe_word.push_str(&format!(
                "_{}",
                existing_names.get(word).copied().unwrap().to_string()
            ));
        }
        safe_word
    } else {
        word.to_lowercase().replace(' ', "_")
    }
}
