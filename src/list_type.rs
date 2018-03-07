use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::Value;
use super::NumericType;

#[derive(Debug, Clone)]
pub struct ListType {
    list: Vec<Value>
}

impl ListType {
    pub fn new(list: Vec<Value>) -> ListType {
        ListType { list }
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
        let (lower, upper) = match (lower, upper) {
            (Some(Value::Number(NumericType::Integer(lower))),
             Some(Value::Number(NumericType::Integer(upper)))) => {
                let lower = calculate_slice(lower, self.list.len());
                let upper = calculate_slice(upper, self.list.len());
                (Some(lower), Some(upper))
            },
            (Some(Value::Number(NumericType::Integer(lower))), None) |
            (Some(Value::Number(NumericType::Integer(lower))),
             Some(Value::None)) => {
                (Some(calculate_slice(lower, self.list.len())), None)
            },
            (None, Some(Value::Number(NumericType::Integer(upper)))) |
            (Some(Value::None), Some(Value::Number(NumericType::
             Integer(upper)))) => {
                (None, Some(calculate_slice(upper, self.list.len())))
            },
            (None, None) | (Some(Value::None), Some(Value::None)) => {
                (None, None)
            },
            _ => panic!("slice indices must be integers or None")
        };

        let list = if step < 0 {
            let list: Vec<Value> = self.list.clone().into_iter()
                .rev().collect();
            // invert lower and upper to apply to the reverse
            let lower = match lower {
                Some(val) => self.list.len() - val - 1,
                None => 0
            };
            let upper = match upper {
                Some(val) => self.list.len() - val - 1,
                None => self.list.len()
            };

            if lower >= upper {
                vec![]
            } else {
                (&list[lower..upper]).to_vec().iter()
                    .enumerate()
                    .filter(|elem| elem.0 % (step.abs() as usize) == 0)
                    .map(|elem| elem.1.clone())
                    .collect()
            }
        } else {
            let lower = match lower {
                Some(val) => val,
                None => 0
            };
            let upper = match upper {
                Some(val) => val,
                None => self.list.len()
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
            output.push_str(&format!("{}", value))
        }

        for value in list_iter {
            output.push_str(&format!(", {}", value));
        }
        output.push_str("]");

        write!(f, "{}", output)
    }
}

/// Takes an index and a list length and calculates the proper usize index,
/// this is necessary since Python allows for negative indexes/slices.
fn calculate_slice(index: i32, len: usize) -> usize {
    if index < 0 {
        if index.abs() as usize >= len {
            0
        } else {
            len - index.abs() as usize
        }
    } else {
        if index as usize >= len {
            len
        } else {
            index as usize
        }
    }
}
