use super::vm::{Const};

#[derive(Debug, Clone)]
pub struct ByteCodeGen {
    pub consts: Const,
}

pub type ByteCode = Vec<u8>;

impl ByteCodeGen {
    pub fn new() -> ByteCodeGen {
        ByteCodeGen {
            consts: Const::new(),
        }
    }
}
