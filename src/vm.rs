pub const END: u8 = 0x00;
pub const CREATE_CONTEXT: u8 = 0x01;
pub const GET_GLOBAL: u8 = 0x02;
pub const GET_MEMBER: u8 = 0x03;
pub const PUSH_CONST: u8 = 0x04;
pub const CALL: u8 = 0x05;

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
    String(String),
}

impl Const {
    pub fn new() -> Const {
        Const {
            value: vec![],
            string: vec![],
        }
    }
}