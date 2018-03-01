mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;
mod list_type;
pub use list_type::ListType;
pub mod builtin;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Looks up a value in the provided scope list. Calling clone() on certain
/// values like Objects and Lists will increase the reference count. This is
/// identical to calling Rc::clone(), it just operates implicitly.
pub fn lookup_value(scope: &Vec<HashMap<String, Value>>, name: &str) -> Value {
    for tbl in scope.iter().rev() {
        if let Some(value) = tbl.get(name) {
            return value.clone()
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

/// Creates a Value::List from a vector of values, this keeps the Cannoli
/// output header nice and clean (no need to include Rc and RefCell)
pub fn create_list(list: Vec<Value>) -> Value {
    Value::List(Rc::new(RefCell::new(ListType::new(list))))
}

/// Creates a Value::Object from a vector of values, this keeps the Cannoli
/// output header nice and clean (no need to include Rc and RefCell)
/// This function is used for modules created by 'import', generally objects
/// are created in Value when a Value::Class is invoked
pub fn create_obj(tbl: HashMap<String, Value>) -> Value {
    Value::Object { tbl: Rc::new(RefCell::new(tbl)) }
}

// If the attribute belongs to a Value::Class, the `self` value is not passed
// through to the function call, if it's a Value::Object the value is passed.
pub fn call_member(value: Value, attr: &str, scope: Vec<HashMap<String, Value>>,
    mut args: Vec<Value>) -> Value {
    match value {
        Value::List(ref list) => {
            list.borrow_mut().call(attr, args)
        },
        Value::Class { ref tbl } => {
            if let Some(func) = tbl.get(attr) {
                func.call(scope, args)
            } else {
                panic!(format!("'class' has no attribute '{}'", attr))
            }
        },
        Value::Object { ref tbl } => {
            // This forces the borrowed `tbl` value to be dropped, without the
            // .clone() on `func` this won't compile. If the func.call() was
            // inside the if-statement we would get a runtime borrow panic.
            let func = if let Some(func) = tbl.borrow().get(attr) {
                func.clone()
            } else {
                panic!(format!("'object' has no attribute '{}'", attr))
            };

            let mut amended_args = vec![value.clone()];
            amended_args.append(&mut args);
            func.call(scope, amended_args)
        },
        _ => unimplemented!()
    }
}
