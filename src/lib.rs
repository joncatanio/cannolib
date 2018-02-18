mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;
pub mod builtin;

use std::collections::HashMap;

pub fn lookup_value(scope: Vec<HashMap<String, Value>>, name: &str) -> Value {
    for tbl in scope.iter().rev() {
        if let Some(value) = tbl.get(name) {
            return value.clone()
        }
    }

    panic!(format!("name '{}' is not defined", name))
}
