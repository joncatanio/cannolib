use super::Value;
use super::NumericType;
use std::collections::HashMap;
use std::rc::Rc;

pub fn import_module() -> Value {
    let mut tbl = HashMap::new();
    tbl.insert("__class__".to_string(), Value::Str("math".to_string()));
    tbl.insert("sqrt".to_string(), Value::Function(Rc::new(sqrt)));
    Value::Class { tbl }
}

fn sqrt(params: Vec<Value>) -> Value {
    if params.is_empty() {
        panic!("sqrt() takes exactly one argument")
    }
    let mut params_iter = params.iter();
    let value = params_iter.next().unwrap();

    match *value {
        Value::Number(NumericType::Integer(val)) => {
            Value::Number(NumericType::Float((val as f32).sqrt()))
        },
        Value::Number(NumericType::Float(val)) => {
            Value::Number(NumericType::Float(val.sqrt()))
        },
        _ => panic!("sqrt() argument must be real number")
    }
}
