mod value;
pub use value::Value;
mod numeric_type;
pub use numeric_type::NumericType;

pub fn cons_int(val: usize) -> Value {
    Value::Number(NumericType::Integer(val))
}

pub fn cons_float(val: f32) -> Value {
    Value::Number(NumericType::Float(val))
}
