use std::ops;

use super::NumericType;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(NumericType),
    Str(String),
    Bool(bool),
    None
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match *self {
            Value::Bool(val) => val,
            _ => unimplemented!()
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match self {
            Value::Number(num) => {
                match other {
                    Value::Number(other_num) => Value::Number(num + other_num),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match self {
            Value::Number(num) => {
                match other {
                    Value::Number(other_num) => Value::Number(num * other_num),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
}
