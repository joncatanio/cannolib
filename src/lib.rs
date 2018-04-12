mod value;
mod types;
pub use value::Value;
pub use types::NumericType;
pub use types::ListType;
pub use types::TupleType;
pub use types::IOWrapper;
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
        Value::Object { ref tbl, ref members } => {
            if let Some(ndx) = tbl.get(attr) {
                members.borrow_mut()[*ndx] = src;
            } else {
                panic!(format!("'object' has no attribute '{}'", attr));
            }
        },
        _ => panic!("cannot access attribute on primitives")
    }
}

// TODO this probably should return a HashMap anymore, this won't be used
// in the optimizations for the paper though, just changing it to compile.
/// Takes an object and a list of (names, aliases) and deconstructs the object
/// into a HashMap will be merged into the local scope list. If None is passed
/// into the 'membs' parameter the entire object is mapped.
pub fn split_object(object: Value, membs: Option<Vec<(String, String)>>)
    -> HashMap<String, Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    let (tbl, members) = match object {
        Value::Object { ref tbl, ref members } => (tbl, members),
        _ => panic!("Value is not 'object'")
    };

    if let Some(membs) = membs {
        for member in membs.iter() {
            let ndx = tbl.get(&member.0).expect(&format!("no member '{}' found",
                member.0));
            map.insert(member.1.clone(), (members.borrow()[*ndx]).clone());
        }
        map
    } else {
        unimplemented!()
    }
}

// If the attribute belongs to a Value::Class, the `self` value is not passed
// through to the function call, if it's a Value::Object the value is passed.
pub fn call_member(mut value: Value, attr: &str, mut args: Vec<Value>,
    kwargs: HashMap<String, Value>) -> Value {
    match value {
        Value::Str(ref string) => {
            // TODO make a string library and write all string methods there
            match attr {
                "split" => {
                    // TODO consider function args
                    let strings = string.split(" ").collect::<Vec<&str>>();
                    let vec: Vec<Value> = strings.iter()
                        .map(|s| Value::Str(s.to_string())).collect();
                    Value::List(Rc::new(RefCell::new(ListType::new(vec))))
                },
                _ => panic!(format!("'str' has no attribute '{}'", attr))
            }
        },
        Value::List(ref list) => {
            list.borrow_mut().call(attr, args, kwargs)
        },
        Value::Class { ref tbl, ref members } => {
            if let Some(ndx) = tbl.get(attr) {
                members[*ndx].call(args, kwargs)
            } else {
                panic!(format!("'class' has no attribute '{}'", attr))
            }
        },
        Value::Object { ref tbl, ref members } => {
            // This forces the borrowed `tbl` value to be dropped, without the
            // .clone() on `func` this won't compile. If the func.call() was
            // inside the if-statement we would get a runtime borrow panic.
            let func = if let Some(ndx) = tbl.get(attr) {
                members.borrow()[*ndx].clone()
            } else {
                panic!(format!("'object' has no attribute '{}'", attr))
            };
            let is_module = match tbl.get("__module__") {
                Some(ndx) => {
                    match members.borrow()[*ndx] {
                        Value::Bool(boolean) => boolean,
                        _ => false
                    }
                },
                None => false
            };
            let args = if is_module {
                args
            } else {
                let mut amended_args = vec![value.clone()];
                amended_args.append(&mut args);
                amended_args
            };

            func.call(args, kwargs)
        },
        Value::TextIOWrapper(ref mut iow) => {
            iow.call(attr, args, kwargs)
        },
        _ => unimplemented!()
    }
}
