use std::collections::HashMap;

/// A simple runtime environment mapping variable names to f64 values.
pub struct Environment {
    values: HashMap<String, f64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    /// Define or reassign a variable
    pub fn define(&mut self, name: String, value: f64) {
        self.values.insert(name, value);
    }

    /// Look up a variable; panic if undefined
    pub fn get(&self, name: &str) -> f64 {
        *self.values.get(name)
            .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
    }
}