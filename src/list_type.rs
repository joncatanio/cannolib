use std::fmt;

use super::Value;

#[derive(Debug, Clone)]
pub struct ListType {
    list: Vec<Value>
}

impl ListType {
    pub fn new(list: Vec<Value>) -> ListType {
        ListType { list }
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
