mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;

// Logical NOT provided as a translation for Python's `not` keyword. Rust only
// provides one op to be overloaded and cannolib has used that as Bitwise NOT.
