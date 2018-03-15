use std::fs::File;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum IOWrapper {
    Stdin,
    Stderr,
    Stdout,
    File(Rc<File>)
}
