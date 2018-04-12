use super::{Value, NumericType};
use std::collections::HashMap;
use std::rc::Rc;

const MOD_SIZE: usize = 2;

pub fn import_module() -> Value {
    let mut tbl = HashMap::new();
    let mut members = Vec::with_capacity(MOD_SIZE);
    members.resize(MOD_SIZE, Value::Undefined);

    let mut ndx = 0;
    tbl.insert("__name__".to_string(), ndx);
    members[ndx] = Value::Str("math".to_string());

    ndx += 1;
    tbl.insert("sqrt".to_string(), ndx);
    members[ndx] = Value::Function(Rc::new(sqrt));

    Value::Class { tbl: Rc::new(tbl), members }
}

fn sqrt(params: Vec<Value>, _kwargs: HashMap<String, Value>) -> Value {
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
