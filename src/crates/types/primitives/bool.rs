use std::string;

pub(crate) struct Boolean {
    name: string::String,
    value: bool,
}

impl Boolean {
    pub fn new(_value: bool) -> Self {
        Boolean {
            name: "Boolean".to_string(),
            value: _value,
        }
    }

    pub fn to_string(&self) -> string::String {
        self.value.to_string()
    }
}
