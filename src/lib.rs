mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;
pub mod builtin;

use std::collections::HashMap;
use std::rc::Rc;

/// Looks up a value in the provided scope list. Abstracts reference logic that
/// provides some of the more dynamic features of Python.
pub fn lookup_value(scope: &Vec<HashMap<String, Value>>, name: &str) -> Value {
    for tbl in scope.iter().rev() {
        if let Some(value) = tbl.get(name) {
            match *value {
                Value::Object { ref tbl } => {
                    return Value::Object { tbl: Rc::clone(tbl) }
                },
                _ => return value.clone()
            }
        }
    }

    panic!(format!("name '{}' is not defined", name))
}

/// Attribute assign modifies the `dest` argument.
pub fn attr_assign(dest: Value, attr: &str, src: Value) {
    match dest {
        Value::Object { ref tbl } => {
            tbl.borrow_mut().insert(attr.to_string(), src);
        },
        _ => panic!("cannot access attribute on primitives")
    }
}

// If the attribute belongs to a Value::Class, the `self` value is not passed
// through to the function call, if it's a Value::Object the value is passed.
pub fn call_member(value: Value, attr: &str, scope: Vec<HashMap<String, Value>>,
    mut args: Vec<Value>) -> Value {
    match value {
        Value::Class { ref tbl } => {
            if let Some(func) = tbl.get(attr) {
                func.call(scope, args)
            } else {
                panic!(format!("'class' has no attribute '{}'", attr))
            }
        },
        Value::Object { ref tbl } => {
            if let Some(func) = tbl.borrow().get(attr) {
                let mut amended_args = vec![value.clone()];
                amended_args.append(&mut args);
                func.call(scope, amended_args)
            } else {
                panic!(format!("'object' has no attribute '{}'", attr))
            }
        },
        _ => unimplemented!()
    }
}
