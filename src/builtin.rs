use super::Value;

pub fn print(params: Vec<Value>) -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next().expect("expected positional arg");
    println!("{}", value);

    Value::None
}
