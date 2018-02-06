use std::ops;
use std::cmp;
use std::fmt;

use super::NumericType;

#[derive(Debug, Clone)]
pub enum Value {
    Number(NumericType),
    Str(String),
    Bool(bool),
    Closure { f: fn(Vec<Value>) -> Value, params: Vec<String> },
    None
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match *self {
            Value::Number(ref val) => val.to_bool(),
            Value::Str(ref val) => if val.is_empty() { false } else { true },
            Value::Bool(ref val) => *val,
            Value::None => false,
            _ => unimplemented!()
        }
    }

    // Logical NOT provided as a translation for Python's `not` keyword. Rust
    // provides one overload NOT and cannolib has used that as Bitwise NOT.
    // This function will always return a Value::Bool.
    pub fn logical_not(&self) -> Value {
        Value::Bool(!self.to_bool())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(ref n) => write!(f, "{}", n),
            Value::Str(ref s)    => write!(f, "{}", s),
            Value::Bool(ref b)   => write!(f, "{}", b),
            Value::None          => write!(f, "None"),
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
            (Value::Str(lhs), Value::Str(rhs)) => {
                Value::Str(lhs + &rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::BitAnd for Value {
    type Output = Value;

    fn bitand(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs & rhs)
            },
            _ => panic!("Bitwise AND applies to Value::Number")
        }
    }
}

impl ops::BitOr for Value {
    type Output = Value;

    fn bitor(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs | rhs)
            },
            _ => panic!("Bitwise OR applies to Value::Number")
        }
    }
}

impl ops::BitXor for Value {
    type Output = Value;

    fn bitxor(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs ^ rhs)
            },
            _ => panic!("Bitwise XOR applies to Value::Number")
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs / rhs)
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

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Number(val) => Value::Number(-val),
            Value::Bool(val) =>
                Value::Number(NumericType::Integer(-(val as i32))),
            _ => panic!("bad operand type for unary -"),
        }
    }
}

impl ops::Not for Value {
    type Output = Value;

    // Bitwise NOT, cannolib has a logical not provided as a function call
    fn not(self) -> Value {
        match self {
            Value::Number(val) => Value::Number(!val),
            Value::Bool(val) =>
                Value::Number(NumericType::Integer(!(val as i32))),
            _ => panic!("bad operand type for unary ~"),
        }
    }
}

impl ops::Rem for Value {
    type Output = Value;

    fn rem(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs % rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Shl<Value> for Value {
    type Output = Value;

    fn shl(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs << rhs)
            },
            _ => unimplemented!()
        }
    }
}

impl ops::Shr<Value> for Value {
    type Output = Value;

    fn shr(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Value::Number(lhs >> rhs)
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
    fn self_to_bool_number() {
        let a = Value::Number(NumericType::Float(1.5));
        let b = Value::Number(NumericType::Float(0.0));
        let x = Value::Number(NumericType::Integer(15));
        let y = Value::Number(NumericType::Integer(0));
        let z = Value::Number(NumericType::Integer(-1));

        assert_eq!(a.to_bool(), true);
        assert_eq!(b.to_bool(), false);
        assert_eq!(x.to_bool(), true);
        assert_eq!(y.to_bool(), false);
        assert_eq!(z.to_bool(), true);
    }

    #[test]
    fn self_to_bool_str() {
        let x = Value::Str("".to_string());
        let y = Value::Str("test".to_string());

        assert_eq!(x.to_bool(), false);
        assert_eq!(y.to_bool(), true);
    }

    #[test]
    fn self_to_bool_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(x.to_bool(), true);
        assert_eq!(y.to_bool(), false);
    }

    #[test]
    fn self_to_bool_none() {
        let x = Value::None;

        assert_eq!(x.to_bool(), false);
    }

    #[test]
    fn self_logical_not_number() {
        let a = Value::Number(NumericType::Float(1.5));
        let b = Value::Number(NumericType::Float(0.0));
        let x = Value::Number(NumericType::Integer(15));
        let y = Value::Number(NumericType::Integer(0));
        let z = Value::Number(NumericType::Integer(-1));

        assert_eq!(a.logical_not(), Value::Bool(false));
        assert_eq!(b.logical_not(), Value::Bool(true));
        assert_eq!(x.logical_not(), Value::Bool(false));
        assert_eq!(y.logical_not(), Value::Bool(true));
        assert_eq!(z.logical_not(), Value::Bool(false));
    }

    #[test]
    fn self_logical_not_str() {
        let x = Value::Str("".to_string());
        let y = Value::Str("test".to_string());

        assert_eq!(x.logical_not(), Value::Bool(true));
        assert_eq!(y.logical_not(), Value::Bool(false));
    }

    #[test]
    fn self_logical_not_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(x.logical_not(), Value::Bool(false));
        assert_eq!(y.logical_not(), Value::Bool(true));
    }

    #[test]
    fn self_logical_not_none() {
        let x = Value::None;

        assert_eq!(x.logical_not(), Value::Bool(true));
    }

    #[test]
    fn partial_eq_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));
        let z = Value::Number(NumericType::Integer(5));
        let none = Value::None;

        assert_eq!(x == z, true);
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

    #[test]
    fn partial_ord_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));

        assert_eq!(x < y, true);
        assert_eq!(y > x, true);
        assert_eq!(x >= x, true);
        assert_eq!(x <= y, true);
    }

    #[test]
    fn partial_ord_value_str() {
        let x = Value::Str("a".to_string());
        let y = Value::Str("z".to_string());

        assert_eq!(x < y, true);
        assert_eq!(x > y, false);
        assert_eq!(x >= y, false);
        assert_eq!(x <= y, true);
    }

    #[test]
    fn partial_ord_value_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(x < y, false);
        assert_eq!(x > y, true);
        assert_eq!(x >= y, true);
        assert_eq!(x <= y, false);
    }

    #[test]
    fn op_add_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));
        let z = Value::Number(NumericType::Float(2.0));

        assert_eq!(x.clone() + y, Value::Number(NumericType::Integer(11)));
        assert_eq!(x.clone() + z, Value::Number(NumericType::Float(7.0)));
    }

    #[test]
    fn op_add_value_str() {
        let x = Value::Str("test".to_string());
        let y = Value::Str("concat".to_string());

        assert_eq!(x + y, Value::Str("testconcat".to_string()));
    }

    #[test]
    fn op_bitand_value_number() {
        let x = Value::Number(NumericType::Integer(1));
        let y = Value::Number(NumericType::Integer(2));

        assert_eq!(x & y, Value::Number(NumericType::Integer(0)))
    }

    #[test]
    fn op_bitor_value_number() {
        let x = Value::Number(NumericType::Integer(1));
        let y = Value::Number(NumericType::Integer(2));

        assert_eq!(x | y, Value::Number(NumericType::Integer(3)))
    }

    #[test]
    fn op_bitxor_value_number() {
        let x = Value::Number(NumericType::Integer(1));
        let y = Value::Number(NumericType::Integer(2));

        assert_eq!(x ^ y, Value::Number(NumericType::Integer(3)))
    }

    #[test]
    fn op_div_value_number() {
        let x = Value::Number(NumericType::Integer(1));
        let y = Value::Number(NumericType::Integer(2));
        let z = Value::Number(NumericType::Float(2.0));

        assert_eq!(x.clone() / y.clone(),
            Value::Number(NumericType::Float(0.5)));
        assert_eq!(x.clone() / z.clone(),
            Value::Number(NumericType::Float(0.5)));
        assert_eq!(y.clone() / x.clone(),
            Value::Number(NumericType::Float(2.0)));
        assert_eq!(z.clone() / z.clone(),
            Value::Number(NumericType::Float(1.0)));
    }

    #[test]
    fn op_mul_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));
        let z = Value::Number(NumericType::Float(2.0));

        assert_eq!(x.clone() * y, Value::Number(NumericType::Integer(30)));
        assert_eq!(x.clone() * z, Value::Number(NumericType::Float(10.0)));
    }

    #[test]
    fn op_neg_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(-2));
        let z = Value::Number(NumericType::Integer(0));

        assert_eq!(-x, Value::Number(NumericType::Integer(-5)));
        assert_eq!(-y, Value::Number(NumericType::Integer(2)));
        assert_eq!(-z, Value::Number(NumericType::Integer(0)));
    }

    #[test]
    fn op_neg_value_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(-x, Value::Number(NumericType::Integer(-1)));
        assert_eq!(-y, Value::Number(NumericType::Integer(0)));
    }

    #[test]
    fn op_not_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(-2));
        let z = Value::Number(NumericType::Integer(0));

        assert_eq!(!x, Value::Number(NumericType::Integer(-6)));
        assert_eq!(!y, Value::Number(NumericType::Integer(1)));
        assert_eq!(!z, Value::Number(NumericType::Integer(-1)));
    }

    #[test]
    fn op_not_value_bool() {
        let x = Value::Bool(true);
        let y = Value::Bool(false);

        assert_eq!(!x, Value::Number(NumericType::Integer(-2)));
        assert_eq!(!y, Value::Number(NumericType::Integer(-1)));
    }

    #[test]
    fn op_rem_value_number() {
        let x = Value::Number(NumericType::Integer(10));
        let y = Value::Number(NumericType::Integer(3));

        assert_eq!(x % y, Value::Number(NumericType::Integer(1)));
    }

    #[test]
    fn op_shl_value_number() {
        let x = Value::Number(NumericType::Integer(128));
        let y = Value::Number(NumericType::Integer(4));

        assert_eq!(x << y, Value::Number(NumericType::Integer(2048)));
    }

    #[test]
    fn op_shr_value_number() {
        let x = Value::Number(NumericType::Integer(64));
        let y = Value::Number(NumericType::Integer(2));
        let a = Value::Number(NumericType::Integer(1));
        let b = Value::Number(NumericType::Integer(5));

        assert_eq!(x >> y, Value::Number(NumericType::Integer(16)));
        assert_eq!(a >> b, Value::Number(NumericType::Integer(0)));
    }

    #[test]
    fn op_sub_value_number() {
        let x = Value::Number(NumericType::Integer(5));
        let y = Value::Number(NumericType::Integer(6));
        let z = Value::Number(NumericType::Float(2.0));

        assert_eq!(x.clone() - y, Value::Number(NumericType::Integer(-1)));
        assert_eq!(x.clone() - z, Value::Number(NumericType::Float(3.0)));
    }
}
