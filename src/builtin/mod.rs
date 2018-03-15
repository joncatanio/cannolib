pub mod sys;
pub mod math;

use super::Value;
use super::NumericType;
use super::ListType;
use super::TupleType;
use super::IOWrapper;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use std::rc::Rc;
use std::cell::RefCell;

pub fn get_scope() -> HashMap<String, Value> {
    let mut tbl = HashMap::new();
    tbl.insert("print".to_string(), Value::Function(Rc::new(print)));
    tbl.insert("str".to_string(), Value::Function(Rc::new(py_str)));
    tbl.insert("len".to_string(), Value::Function(Rc::new(len)));
    tbl.insert("min".to_string(), Value::Function(Rc::new(min)));
    tbl.insert("int".to_string(), Value::Function(Rc::new(int)));
    tbl.insert("float".to_string(), Value::Function(Rc::new(float)));
    tbl.insert("enumerate".to_string(), Value::Function(Rc::new(enumerate)));
    tbl.insert("open".to_string(), Value::Function(Rc::new(open)));
    tbl
}

fn print(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => print!("{}", val),
        None => ()
    }

    for value in params_iter {
        print!(" {}", value)
    }

    println!();
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

// TODO implement base keyword arg
pub fn int(params:Vec<Value>) -> Value {
    if params.is_empty() {
        return Value::Number(NumericType::Integer(0))
    }
    let mut params_iter = params.iter();
    let value = params_iter.next().unwrap();

    match *value {
        Value::Str(ref string) => {
            if let Ok(val) = string.trim().parse::<i32>() {
                Value::Number(NumericType::Integer(val))
            } else {
                panic!("could not convert string to int")
            }
        },
        Value::Number(NumericType::Integer(_)) => value.clone(),
        Value::Number(NumericType::Float(val)) => {
            Value::Number(NumericType::Integer(val as i32))
        },
        _ => panic!("int() argument must be a string or a number")
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

/// This function is currently very different from the Python3 implementation
/// it is not a generator. It will output a Value::List instead of an enumerate
/// object that calls the iterable's '__next__' function. Ideally it will be
/// changed to function like Python3.
pub fn enumerate(params: Vec<Value>) -> Value {
    if params.is_empty() {
        panic!("enumerate() takes at most 2 arguments, 0 were given")
    }
    let mut params_iter = params.iter();
    let value = params_iter.next().unwrap();
    let mut start: i32 = if let Some(val) = params_iter.next() {
        match *val {
            Value::Number(NumericType::Integer(i)) => i,
            _ => panic!("enumerate() 'start' must be integer")
        }
    } else {
        0
    };

    let vec: Vec<Value> = match *value {
        Value::Str(ref string) => {
            string.chars().map(|c| {
                let tup = Value::Tuple(TupleType::new(vec![
                    Value::Number(NumericType::Integer(start)),
                    Value::Str(c.to_string())
                ]));
                start += 1;
                tup
            }).collect()
        },
        Value::List(ref list) => {
            list.borrow().clone_seq().iter().map(|x| {
                let tup = Value::Tuple(TupleType::new(vec![
                    Value::Number(NumericType::Integer(start)),
                    x.clone()
                ]));
                start += 1;
                tup
            }).collect()

        },
        Value::Tuple(ref tup) => {
            tup.clone_seq().iter().map(|x| {
                let tup = Value::Tuple(TupleType::new(vec![
                    Value::Number(NumericType::Integer(start)),
                    x.clone()
                ]));
                start += 1;
                tup
            }).collect()
        },
        Value::TextIOWrapper(ref iow) => {
            let file = match *iow {
                IOWrapper::File(ref file) => file,
                _ => panic!("unsupported operation")
            };
            let reader = BufReader::new(&**file);
            reader.lines().map(|x| {
                let mut string = x.unwrap().to_string();
                string.push_str("\n");
                let tup = Value::Tuple(TupleType::new(vec![
                    Value::Number(NumericType::Integer(start)),
                    Value::Str(string)
                ]));
                start += 1;
                tup
            }).collect()
        },
        _ => panic!("enumerate() value not iterable")
    };

    Value::List(Rc::new(RefCell::new(ListType::new(vec))))
}

pub fn open(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let filename = match params_iter.next() {
        Some(&Value::Str(ref s)) => s,
        Some(_) => unimplemented!(),
        None => panic!("bad 'file' parameter specified")
    };
    let mode = match params_iter.next() {
        Some(&Value::Str(ref s)) => s,
        Some(_) => panic!("'mode' must be a str"),
        None => "r"
    };
    let mode_r = mode.contains("r");
    let mode_w = mode.contains("w");
    let result = OpenOptions::new()
        .read(mode_r)
        .write(mode_w)
        .truncate(mode_w)
        .create(mode_w)
        .open(filename);
    let file = match result {
        Ok(file) => file,
        Err(err) => panic!("error opening file '{}': {}", filename, err)
    };

    Value::TextIOWrapper(IOWrapper::File(Rc::new(file)))
}
