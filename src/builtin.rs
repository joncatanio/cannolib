use super::Value;
use super::NumericType;
use std::collections::HashMap;
use std::rc::Rc;

pub fn get_scope() -> HashMap<String, Value> {
    let mut tbl = HashMap::new();
    tbl.insert("print".to_string(), Value::Function(Rc::new(print)));
    tbl.insert("str".to_string(), Value::Function(Rc::new(py_str)));
    tbl.insert("len".to_string(), Value::Function(Rc::new(len)));
    tbl.insert("min".to_string(), Value::Function(Rc::new(min)));
    tbl.insert("float".to_string(), Value::Function(Rc::new(float)));
    tbl
}

fn print(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => println!("{}", val),
        None => println!()
    }

    Value::None
}

pub fn py_str(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => Value::Str(val.to_string()),
        None => Value::Str("".to_string())
    }
}

pub fn len(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let value = match params_iter.next() {
        Some(value) => value,
        None => panic!("len() takes exactly one argument")
    };

    match *value {
        Value::Str(ref string) =>
            Value::Number(NumericType::Integer(string.len() as i32)),
        Value::List(ref list) => list.borrow().len(),
        Value::Tuple(ref tup) => tup.len(),
        _ => panic!("value has no len()")
    }
}

// If one positional argument is provided, it should be an iterable, otherwise
// each positional argument will be compared to each other.
// TODO probably should implement std::cmp::Ord on Value to get min/max
pub fn min(params: Vec<Value>) -> Value {
    if params.len() == 1 {
        let mut params_iter = params.iter();
        let value = params_iter.next().unwrap();

        match *value {
            Value::Str(ref string) => {
                if string.is_empty() {
                    panic!("min() arg is an empty sequence")
                }
                Value::Str(string.chars().min().unwrap().to_string())
            },
            Value::List(ref list) => list.borrow().min(),
            Value::Tuple(ref tup) => tup.min(),
            _ => panic!("min(): value is not iterable")
        }
    } else {
        if params.is_empty() {
            panic!("min expected 1 argument, 0 were supplied")
        }
        let mut iter = params.iter();
        let mut min_val = iter.next().unwrap();

        for value in iter {
            if value < min_val {
                min_val = value
            }
        }

        min_val.clone()
    }
}

/// Converts a string in base 10 to a float. Or numerical values to floats.
/// Accepts an optional decimal exponent. This function accepts strings such as:
///
/// * '3.14'
/// * '-3.14'
/// * '2.5E10', or equivalently, '2.5e10'
/// * '2.5E-10'
/// * '5.'
/// * '.5', or, equivalently, '0.5'
///
/// Leading and trailing whitespace is trimmed before parsing
pub fn float(params: Vec<Value>) -> Value {
    if params.is_empty() {
        return Value::Number(NumericType::Float(0.0))
    }
    let mut params_iter = params.iter();
    let value = params_iter.next().unwrap();

    match *value {
        Value::Str(ref string) => {
            if let Ok(val) = string.trim().parse::<f32>() {
                Value::Number(NumericType::Float(val))
            } else {
                panic!("could not convert string to float")
            }
        },
        Value::Number(NumericType::Integer(val)) => {
            Value::Number(NumericType::Float(val as f32))
        },
        Value::Number(NumericType::Float(_)) => value.clone(),
        _ => panic!("float() argument must be a string or a number")
    }
}
