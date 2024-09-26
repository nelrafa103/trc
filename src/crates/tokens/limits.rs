fn is_rust_delimiter(s: &str) -> bool {
    match s {
        "(" | ")" | "[" | "]" | "{" | "}" | "," | ";" | "." | ":" | "::" | "->" | "=>" | "#"
        | "!" => true,
        _ => false,
    }
}
