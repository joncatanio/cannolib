use std::fmt;
use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

use ::Value;
use super::NumericType;

#[derive(Debug, Clone)]
pub struct ListType {
    list: Vec<Value>
}

impl ListType {
    pub fn new(list: Vec<Value>) -> ListType {
        ListType { list }
    }

    pub fn to_bool(&self) -> bool {
        !(self.list.is_empty())
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

    pub fn contains(&self, value: &Value) -> bool {
        self.list.iter().any(|e| e == value)
    }

    /// Returns the length of the list
    pub fn len(&self) -> Value {
        Value::Number(NumericType::Integer(self.list.len() as i32))
    }

    /// Returns minimum element in sequence, will fail if elements in sequence
    /// vary in type.
    pub fn min(&self) -> Value {
        if self.list.is_empty() {
            panic!("min() arg is an empty sequence")
        }
        let mut iter = self.list.iter();
        let mut min_val = iter.next().unwrap();

        for value in iter {
            if value < min_val {
                min_val = value
            }
        }

        min_val.clone()
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

        Value::List(Rc::new(RefCell::new(ListType::new(list))))
    }

    pub fn clone_seq(&self) -> Vec<Value> {
        self.list.clone()
    }

    pub fn call(&mut self, attr: &str, args: Vec<Value>) -> Value {
        match attr {
            "append" => self.append(args),
            _ => panic!(format!("'list' has no attribute '{}'", attr))
        }
    }

    fn append(&mut self, mut args: Vec<Value>) -> Value {
        if args.len() != 1 {
            panic!(format!("append() takes exactly one argument ({} given)",
                args.len()));
        }

        self.list.push(args.pop().unwrap());
        Value::None
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut list_iter = self.list.iter();

        output.push_str("[");
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
        output.push_str("]");

        write!(f, "{}", output)
    }
}

impl cmp::PartialEq for ListType {
    fn eq(&self, other: &ListType) -> bool {
        self.list == other.list
    }

    fn ne(&self, other: &ListType) -> bool {
        self.list != other.list
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
