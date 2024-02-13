use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(v) =>  write!(f, "{}", v),
        }
    }
}