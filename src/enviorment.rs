use std::collections::HashMap;
use crate::helper::Value;

struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Self { values: HashMap::new() }
    }

    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}
