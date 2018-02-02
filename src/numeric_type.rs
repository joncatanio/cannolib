use std::ops;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum NumericType {
    Integer(usize),
    Float(f32)
}

impl NumericType {
    pub fn to_bool(&self) -> bool {
        match *self {
            NumericType::Integer(ref val) =>
                if *val == 0 { false } else { true },
            NumericType::Float(ref val) =>
                if *val == 0.0 { false } else { true }
        }
    }
}

impl ops::Add for NumericType {
    type Output = NumericType;

    fn add(self, other: NumericType) -> NumericType {
        match self {
            NumericType::Integer(i1) => {
                match other {
                    NumericType::Integer(i2) => NumericType::Integer(i1 + i2),
                    NumericType::Float(f2) => NumericType::Float(i1 as f32 + f2)
                }
            },
            NumericType::Float(f1) => {
                match other {
                    NumericType::Integer(i2) =>
                        NumericType::Float(f1 + i2 as f32),
                    NumericType::Float(f2) => NumericType::Float(f1 + f2)
                }
            }
        }
    }
}

impl ops::Mul for NumericType {
    type Output = NumericType;

    fn mul(self, other: NumericType) -> NumericType {
        match self {
            NumericType::Integer(i1) => {
                match other {
                    NumericType::Integer(i2) => NumericType::Integer(i1 * i2),
                    NumericType::Float(f2) => NumericType::Float(i1 as f32 * f2)
                }
            },
            NumericType::Float(f1) => {
                match other {
                    NumericType::Integer(i2) =>
                        NumericType::Float(f1 * i2 as f32),
                    NumericType::Float(f2) => NumericType::Float(f1 * f2)
                }
            }
        }
    }
}
