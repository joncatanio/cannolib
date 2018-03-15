use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Value;

#[derive(Debug, Clone)]
pub enum IOWrapper {
    File(Rc<RefCell<File>>),
    Stdin,
    Stderr,
    Stdout,
    Closed
}

impl IOWrapper {
    pub fn call(&mut self, attr: &str, _args: Vec<Value>,
        _kwargs: HashMap<String, Value>) -> Value {
        match attr {
            "close" => self.close(),
            _ => panic!(format!("'list' has no attribute '{}'", attr))
        }
    }

    fn close(&mut self) -> Value {
        match *self {
            IOWrapper::File(ref file) => {
                drop(file)
            },
            _ => ()
        }
        Value::None
    }
}
