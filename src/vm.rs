pub const END: u8 = 0x00;
pub const CREATE_CONTEXT: u8 = 0x01;

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