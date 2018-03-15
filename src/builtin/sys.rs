use super::Value;
use super::ListType;
use std::env;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub fn import_module() -> Value {
    let mut tbl = HashMap::new();
    tbl.insert("__name__".to_string(), Value::Str("sys".to_string()));
    tbl.insert("argv".to_string(), setup_argv());
    Value::Class { tbl }
}

fn setup_argv() -> Value {
    let args: Vec<_> = env::args().map(|x| Value::Str(x)).collect();
    Value::List(Rc::new(RefCell::new(ListType::new(args))))
}
