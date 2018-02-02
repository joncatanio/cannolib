use std::ops;
use std::cmp;

use super::NumericType;

#[derive(Debug, Clone)]
pub enum Value {
    Number(NumericType),
    Str(String),
    Bool(bool),
    None
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match *self {
            Value::Bool(ref val)   => *val,
            Value::Number(ref val) => val.to_bool(),
            _ => unimplemented!()
        }
    }
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 == val2
            },
            (&Value::Number(_), &Value::None) => false,
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 == val2
            },
            (&Value::Str(_), &Value::None) => false,
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 == val2
            },
            (&Value::Bool(_), &Value::None) => false,
            (&Value::None, &Value::None) => true,
            _ => unimplemented!()
        }
    }

    fn ne(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 != val2
            },
            (&Value::Number(_), &Value::None) => true,
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 != val2
            },
            (&Value::Str(_), &Value::None) => true,
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 != val2
            },
            (&Value::Bool(_), &Value::None) => true,
            (&Value::None, &Value::None) => false,
            _ => unimplemented!()
        }
    }
}

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<cmp::Ordering> {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1.partial_cmp(val2)
            },
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1.partial_cmp(val2)
            },
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1.partial_cmp(val2)
            },
            _ => unimplemented!()
        }
    }

    fn lt(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 < val2
            },
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 < val2
            },
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 < val2
            },
            _ => unimplemented!()
        }
    }

    fn le(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 <= val2
            },
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 <= val2
            },
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 <= val2
            },
            _ => unimplemented!()
        }
    }

    fn gt(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 > val2
            },
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 > val2
            },
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 > val2
            },
            _ => unimplemented!()
        }
    }

    fn ge(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(ref val1), &Value::Number(ref val2)) => {
                val1 >= val2
            },
            (&Value::Str(ref val1), &Value::Str(ref val2)) => {
                val1 >= val2
            },
            (&Value::Bool(ref val1), &Value::Bool(ref val2)) => {
                val1 >= val2
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs + rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs * rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs - rhs)
            },
            _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));
        let none = Value::None;

        assert_eq!(x == x, true);
        assert_eq!(x == y, false);
        assert_eq!(x != y, true);
        assert_eq!(x == none, false);
        assert_eq!(x != none, true);
    }

    #[test]
    fn partial_eq_value_str() {
        let x = Value::Str("test".to_string());
        let y = Value::Str("word".to_string());

        assert_eq!(x == x, true);
        assert_eq!(x == y, false);
        assert_eq!(x != y, true);
    }

    #[test]
    fn partial_eq_value_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(x == x, true);
        assert_eq!(x == y, false);
        assert_eq!(x != y, true);
    }

    #[test]
    fn partial_eq_value_none() {
        let x = Value::Bool(true);
        let none = Value::None;

        assert_eq!(none == none, true);
        assert_eq!(x == none, false);
        assert_eq!(x != none, true);
    }
}
