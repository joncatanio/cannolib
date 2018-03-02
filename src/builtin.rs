use super::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub fn get_scope() -> HashMap<String, Value> {
    let mut tbl = HashMap::new();
    tbl.insert("print".to_string(), Value::Function(Rc::new(print)));
    tbl.insert("str".to_string(), Value::Function(Rc::new(py_str)));
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

pub fn py_str(params: Vec<Value>)
    -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => Value::Str(val.to_string()),
        None => Value::Str("".to_string())
    }
}
