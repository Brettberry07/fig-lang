use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl Type {
    // pub fn is_numeric(&self) -> bool {
    //     matches!(self, Type::Int(_) | Type::Float(_))
    // }

    // pub fn is_string(&self) -> bool {
    //     matches!(self, Type::Str(_))
    // }

    // pub fn is_bool(&self) -> bool {
    //     matches!(self, Type::Bool(_))
    // }

    pub fn is_null(&self) -> bool {
        matches!(self, Type::Null)
    }

    // Addition operator for Type
    pub fn add(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a + b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 + b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a + b as f64),

            (Type::Str(a), Type::Str(b)) => Type::Str(a + &b),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    
    // subtraction operator for Type
    pub fn subtract(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a - b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 - b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a - b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    // multiplication operator for Type
    pub fn multiply(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a * b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a * b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 * b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a * b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    // division operator for Type
    pub fn divide(self, other: Type) -> Type {
        if other.is_null() {
            panic!("Division by zero");
        }
        if let Type::Int(0) | Type::Float(0.0) = other {
            panic!("Division by zero");
        }
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a / b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a / b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 / b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a / b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }

    // equality operator for Type
    pub fn equal(self, other: Type) -> Type {
        Type::Bool(self == other)
    }

    // not equal operator for Type
    pub fn not_equal(self, other: Type) -> Type {
        Type::Bool(self != other)
    }

    // < operator for Type
    pub fn less_than(self, other: Type) -> Type {
        match (self, other) {
            // number comparisons
            (Type::Int(a), Type::Int(b)) => Type::Bool(a < b),
            (Type::Float(a), Type::Float(b)) => Type::Bool(a < b),
            (Type::Int(a), Type::Float(b)) => Type::Bool((a as f64) < b),
            (Type::Float(a), Type::Int(b)) => Type::Bool(a < (b as f64)),

            // string comparisons
            (Type::Str(a), Type::Str(b)) => {
                let len_a = a.len() as f64;
                let len_b = b.len() as f64;
                Type::Bool(len_a < len_b)
            }
            _ => panic!("Invalid types for less than"),  // Handle other cases as needed
        }
    }

    // > operator for Type
    pub fn greater_than(self, other: Type) -> Type {
        match (self, other) {
            // number comparisons
            (Type::Int(a), Type::Int(b)) => Type::Bool(a > b),
            (Type::Float(a), Type::Float(b)) => Type::Bool(a > b),
            (Type::Int(a), Type::Float(b)) => Type::Bool((a as f64) > b),
            (Type::Float(a), Type::Int(b)) => Type::Bool(a > (b as f64)),

            // String comparisons
            (Type::Str(a), Type::Str(b)) => {
                let len_a = a.len() as f64;
                let len_b = b.len() as f64;
                Type::Bool(len_a > len_b)
            }

            _ => panic!("Invalid types for greater than"),  // Handle other cases as needed
        }
    }

    // <= operator for Type
    pub fn less_than_equal(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Bool(a <= b),
            (Type::Float(a), Type::Float(b)) => Type::Bool(a <= b),
            (Type::Int(a), Type::Float(b)) => Type::Bool((a as f64) <= b),
            (Type::Float(a), Type::Int(b)) => Type::Bool(a <=(b as f64)),
            _ => panic!("Invalid types for less than equal"),  // Handle other cases as needed
        }
    }
    // >= operator for Type
    pub fn greater_than_equal(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Bool(a >= b),
            (Type::Float(a), Type::Float(b)) => Type::Bool(a >= b),
            (Type::Int(a), Type::Float(b)) => Type::Bool((a as f64) >= b),
            (Type::Float(a), Type::Int(b)) => Type::Bool(a >= (b as f64)),
            _ => panic!("Invalid types for greater than equal"),  // Handle other cases as needed
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Str(s) => write!(f, "{}", s),
            Type::Int(i) => write!(f, "{}", i),
            Type::Bool(b) => write!(f, "{}", b),
            Type::Float(n) => write!(f, "{}", n),
            Type::Null => write!(f, "null"),
        }
    }
}

