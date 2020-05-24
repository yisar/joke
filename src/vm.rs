#[derive(Debug, Clone)]
pub struct Const {
    pub value: Vec<Value>,
    pub string: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Undefined,
    Bool(bool),
    Number(f64),
}

impl Const {
    pub fn new() -> Const {
        Const {
            value: vec![],
            string: vec![],
        }
    }
}