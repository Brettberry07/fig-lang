use std::collections::HashMap;
use crate::types::Type;
use crate::helper::Stmt;
use std::rc::Rc;
use std::cell::RefCell;
/// A simple runtime environment mapping variable names to Types.
pub struct Environment {
    values: HashMap<String, Type>,
    parent: Option<Rc<RefCell<Environment>>>,
    functions: HashMap<String, Function>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            values: HashMap::new(),
            parent: None,
            functions: HashMap::new(),
         }
    }

    /// Create a new scope that references an existing environment as its parent
    pub fn new_scope(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(parent),
            functions: HashMap::new(),
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

    pub fn define_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        if let Some(func) = self.functions.get(name) {
            return Some(func.clone());
        }
        if let Some(ref parent) = self.parent {
            return parent.borrow().get_function(name);
        }
        None
    }
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Stmt,
    pub closure: Rc<RefCell<Environment>>,
}
