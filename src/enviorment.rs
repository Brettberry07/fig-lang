use std::collections::HashMap;
use crate::helper::{Type};
/// A simple runtime environment mapping variable names to f64 values.
pub struct Environment {
    values: HashMap<String, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    /// Define or reassign a variable
    pub fn define(&mut self, name: String, value: Type) {
        self.values.insert(name, value);
    }

    /// Look up a variable; panic if undefined
    pub fn get(&self, name: &str) -> Type {
        self.values.get(name).cloned().unwrap_or_else(|| {
            panic!("Undefined variable: {}", name)
        })
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn update(&mut self, name: String, value: Type) {
        self.values.insert(name, value);
    }
}