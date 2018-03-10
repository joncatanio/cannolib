mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;
mod list_type;
pub use list_type::ListType;
mod tuple_type;
pub use tuple_type::TupleType;
pub mod builtin;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Looks up a value in the provided scope list. Calling clone() on certain
/// values like Objects and Lists will increase the reference count. This is
/// identical to calling Rc::clone(), it just operates implicitly.
pub fn lookup_value(scope: &Vec<Rc<RefCell<HashMap<String, Value>>>>,
    name: &str) -> Value {
    for tbl in scope.iter().rev() {
        if let Some(value) = tbl.borrow().get(name) {
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

/// Takes an object and a list of (names, aliases) and deconstructs the object
/// into a HashMap will be merged into the local scope list. If None is passed
/// into the 'members' parameter the entire object is mapped.
pub fn split_object(object: Value, members: Option<Vec<(String, String)>>)
    -> HashMap<String, Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    let tbl = match object {
        Value::Object { ref tbl } => tbl,
        _ => panic!("Value is not 'object'")
    };

    if let Some(members) = members {
        for member in members.iter() {
            let value = match tbl.borrow().get(&member.0) {
                Some(value) => value.clone(),
                None => panic!(format!("no member '{}' found", member.0))
            };

            map.insert(member.1.clone(), value);
        }
        map
    } else {
        tbl.borrow().clone()
    }
}

// If the attribute belongs to a Value::Class, the `self` value is not passed
// through to the function call, if it's a Value::Object the value is passed.
pub fn call_member(value: Value, attr: &str, mut args: Vec<Value>) -> Value {
    match value {
        Value::List(ref list) => {
            list.borrow_mut().call(attr, args)
        },
        Value::Class { ref tbl } => {
            if let Some(func) = tbl.get(attr) {
                func.call(args)
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
            func.call(amended_args)
        },
        _ => unimplemented!()
    }
}
