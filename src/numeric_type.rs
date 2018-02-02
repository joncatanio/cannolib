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
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs + rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 + rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs + rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs + rhs)
            }
        }
    }
}

impl ops::Mul for NumericType {
    type Output = NumericType;

    fn mul(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs * rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 * rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs * rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs * rhs)
            }
        }
    }
}

impl ops::Sub for NumericType {
    type Output = NumericType;

    fn sub(self, other: NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Integer(lhs), NumericType::Integer(rhs)) => {
                NumericType::Integer(lhs - rhs)
            },
            (NumericType::Integer(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs as f32 - rhs)
            },
            (NumericType::Float(lhs), NumericType::Integer(rhs)) => {
                NumericType::Float(lhs - rhs as f32)
            },
            (NumericType::Float(lhs), NumericType::Float(rhs)) => {
                NumericType::Float(lhs - rhs)
            }
        }
    }
}
