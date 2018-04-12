use super::{Value, NumericType, ListType, IOWrapper};
use std::env;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::process;

const MOD_SIZE: usize = 4;

pub fn import_module() -> Value {
    let mut tbl = HashMap::new();
    let mut members = Vec::with_capacity(MOD_SIZE);
    members.resize(MOD_SIZE, Value::Undefined);

    let mut ndx = 0;
    tbl.insert("__name__".to_string(), ndx);
    members[ndx] = Value::Str("sys".to_string());

    ndx += 1;
    tbl.insert("argv".to_string(), ndx);
    members[ndx] = setup_argv();

    ndx += 1;
    tbl.insert("stderr".to_string(), ndx);
    members[ndx] = Value::TextIOWrapper(IOWrapper::Stderr);

    ndx += 1;
    tbl.insert("exit".to_string(), ndx);
    members[ndx] = Value::Function(Rc::new(py_exit));

    Value::Class { tbl: Rc::new(tbl), members }
}

fn setup_argv() -> Value {
    let args: Vec<_> = env::args().map(|x| Value::Str(x)).collect();
    Value::List(Rc::new(RefCell::new(ListType::new(args))))
}

/// This function is actually a diverging function, to satisfy the
/// cannolib::Value::Function requirements we say that it returns a Value.
fn py_exit(params: Vec<Value>, _kwargs: HashMap<String, Value>) -> Value {
    let mut params_iter = params.iter();
    let value = params_iter.next();
    let exit_status = match value {
        Some(&Value::Number(NumericType::Integer(val))) => val,
        Some(_) => panic!("exit(): exit status must be integer"),
        None => 0
    };

    process::exit(exit_status)
}
