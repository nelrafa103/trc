use std::collections::LinkedList;
use std::string;

pub(crate) struct String {
    name: string::String,
    length: i64,
    value: string::String,
}

impl String {
    pub fn new() -> Self {
        String {
            name: "String".to_string(),
            length: i64::MAX,
            value: "".to_string(),
        }
    }

    pub fn push(&self, concat: string::String) -> string::String {
        self.value.push_str(concat);
    }

    pub fn replace(&self, from: &str, to: &str) -> string::String {
        self.value.replace(from, to).to_string()
    }

    pub fn ends_with(&self, search_string: &str) -> string::String {
        self.value.ends_with(search_string).to_string()
    }

    pub fn concat(&mut self, strings: LinkedList<string::String>) {
        for item in strings {
            self.value.push_str(&item)
        }
    }

    pub fn search(&self, string: string::String) {
        self.value.matches(&string);
    }

    pub fn start_with(&self, search_string: &str) -> string::String {
        self.value.starts_with(search_string).to_string()
    }

    pub fn slice(&self, begin: usize, end: usize) -> string::String {
        self.value[begin..end].to_string()
        //  self.value.(begin, end).to_string()
    }

    pub fn char_at(&self, index: usize) -> string::String {
        self.value.chars().nth(index).unwrap().to_string()
    }

    pub fn to_upper_case(&self) -> string::String {
        self.value.to_uppercase().to_string()
    }

    pub fn to_lower_case(&self) -> string::String {
        self.value.to_lowercase().to_string()
    }

    pub fn to_string(&self) -> string::String {
        self.value.to_string()
    }

    pub fn value_of(&self) -> string::String {
        self.value.to_string()
    }

    pub fn trim(&self) -> string::String {
        self.value.trim().to_string()
    }

    pub fn trim_end(&self) -> string::String {
        self.value.trim_end().to_string()
    }

    pub fn trim_start(&self) -> string::String {
        self.value.trim_start().to_string()
    }

    pub fn to_str(&self) -> &str {
        self.value.as_str()
    }
    // pub fn
}
