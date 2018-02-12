use super::Value;
use std::collections::HashMap;

#[allow(const_err)]
pub fn get_scope() -> HashMap<String, Value> {
    let mut tbl = HashMap::new();
    tbl.insert("print".to_string(), Value::Function { f: print });
    tbl
}

#[allow(const_err)]
fn print(_scope: Vec<HashMap<String, Value>>, params: Vec<Value>)
    -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => println!("{}", val),
        None => println!()
    }

    Value::None
}
