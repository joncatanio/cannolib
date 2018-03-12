use std::fmt;

use super::Value;
use super::NumericType;

#[derive(Debug, Clone)]
pub struct TupleType {
    list: Vec<Value>
}

impl TupleType {
    pub fn new(list: Vec<Value>) -> TupleType {
        TupleType { list }
    }

    pub fn index(&self, index: Value) -> Value {
        let pos = match index {
            Value::Number(NumericType::Integer(pos)) => pos,
            _ => panic!("list indices must be integers or slices")
        };

        if pos.abs() as usize >= self.list.len() {
            panic!("list index out of range")
        }

        let pos: usize = if pos < 0 {
            self.list.len() - pos.abs() as usize
        } else {
            pos as usize
        };

        self.list[pos].clone()
    }

    /// Replicates Python3's slicing feature
    ///
    /// a[start:end] # items start through end-1
    /// a[start:]    # items start through the rest of the array
    /// a[:end]      # items from the beginning through end-1
    /// a[:]         # a copy of the whole array
    ///
    /// a[start:end:step] # start through not past end, by step
    ///
    /// The other feature is that start or end may be a negative number, which
    /// means it counts from the end of the array instead of the beginning.
    /// reference: https://stackoverflow.com/a/509295
    pub fn slice(&self, lower: Option<Value>, upper: Option<Value>,
        step: Option<Value>) -> Value {
        let step = match step {
            Some(Value::Number(NumericType::Integer(step))) => {
                if step == 0 {
                    panic!("slice step cannot be zero")
                } else {
                    step
                }
            },
            Some(Value::None) => 1,
            None => 1,
            _ => panic!("slice indices must be integers or None")
        };
        let list = if step < 0 {
            let lower = match lower {
                Some(Value::Number(NumericType::Integer(lower))) =>
                    calculate_slice(lower, 0, self.list.len() as i32),
                None => self.list.len() as i32,
                _ => panic!("slice indices must be integers or None")
            };
            let upper = match upper {
                Some(Value::Number(NumericType::Integer(upper))) =>
                    calculate_slice(upper, -1, self.list.len() as i32),
                None => -1,
                _ => panic!("slice indices must be integers or None")
            };

            if lower > upper {
                let temp = lower;
                let lower = (upper + 1) as usize;
                let upper = if temp == self.list.len() as i32 { temp as usize }
                    else { (temp + 1) as usize };

                let list: Vec<Value> = (&self.list[lower..upper]).to_vec()
                    .iter().map(|val| val.clone()).collect();

                list.iter().rev().enumerate()
                    .filter(|elem| elem.0 % (step.abs() as usize) == 0)
                    .map(|elem| elem.1.clone())
                    .collect()
            } else {
                vec![]
            }
        } else {
            let lower = match lower {
                Some(Value::Number(NumericType::Integer(lower))) =>
                    calculate_slice(lower, 0, self.list.len() as i32) as usize,
                None => 0,
                _ => panic!("slice indices must be integers or None")
            };
            let upper = match upper {
                Some(Value::Number(NumericType::Integer(upper))) =>
                    calculate_slice(upper, 0, self.list.len() as i32) as usize,
                None => self.list.len(),
                _ => panic!("slice indices must be integers or None")
            };

            if lower >= upper {
                vec![]
            } else {
                (&self.list[lower..upper]).to_vec().iter()
                    .enumerate()
                    .filter(|elem| elem.0 % (step as usize) == 0)
                    .map(|elem| elem.1.clone())
                    .collect()
            }
        };

        Value::Tuple(TupleType::new(list))
    }

    pub fn clone_seq(&self) -> Vec<Value> {
        self.list.clone()
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut list_iter = self.list.iter().peekable();

        output.push_str("(");
        if let Some(value) = list_iter.next() {
            match *value {
                Value::Str(ref s) => {
                    if s.contains("'") {
                        output.push_str(&format!("\"{}\"", value))
                    } else {
                        output.push_str(&format!("'{}'", value))
                    }
                },
                _ => output.push_str(&format!("{}", value))
            }

            if let None = list_iter.peek() {
                output.push_str(",");
            } else {
                for value in list_iter {
                    match *value {
                        Value::Str(ref s) => {
                            if s.contains("'") {
                                output.push_str(&format!(", \"{}\"", value))
                            } else {
                                output.push_str(&format!(", '{}'", value))
                            }
                        },
                        _ => output.push_str(&format!(", {}", value))
                    }
                }
            }
        }
        output.push_str(")");

        write!(f, "{}", output)
    }
}

/// Takes an index and lower/upper bounds and calculates the proper index,
/// this is necessary since Python allows for negative indexes/slices.
fn calculate_slice(index: i32, lower: i32, upper: i32) -> i32 {
    if index < 0 {
        if index.abs() >= upper {
            lower
        } else {
            upper - index.abs()
        }
    } else {
        if index >= upper{
            upper
        } else {
            index
        }
    }
}
