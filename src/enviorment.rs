use std::collections::HashMap;
use crate::types::Type;
use std::rc::Rc;
use std::cell::RefCell;
/// A simple runtime environment mapping variable names to Types.
pub struct Environment {
    values: HashMap<String, Type>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            values: HashMap::new(),
            parent: None,
         }
    }

    /// Create a new scope that references an existing environment as its parent
    pub fn new_scope(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define or reassign a variable
    pub fn define(&mut self, name: String, value: Type) {
        self.values.insert(name, value);
    }

    /// Look up a variable; search current scope, then parents
    pub fn get(&self, name: &str) -> Type {
        if let Some(val) = self.values.get(name) {
            return val.clone();
        }
        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name);
        }
        panic!("Undefined variable: {}", name);
    }

    /// Check if a variable is defined in the current or parent scopes
    pub fn is_defined(&self, name: &str) -> bool {
        if self.values.contains_key(name) {
            return true;
        }
        if let Some(ref parent) = self.parent {
            return parent.borrow().is_defined(name);
        }
        false
    }

    /// Update a variable in the closest scope it’s defined in
    pub fn update(&mut self, name: String, value: Type) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }
        if let Some(ref parent) = self.parent {
            parent.borrow_mut().update(name, value);
            return;
        }
        panic!("Undefined variable: {}", name);
    }
}
