use super::Value;

#[allow(const_err)]
pub static PRINT: Value = Value::Function { f: |params: Vec<Value>| -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();

    match value {
        Some(val) => println!("{}", val),
        None => println!()
    }

    Value::None
}};
