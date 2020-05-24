use super::bytecode::{ByteCodeGen};

#[derive(Debug,Clone)]
pub struct CodeGen {
    bytecode: ByteCodeGen
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            bytecode: ByteCodeGen::new(),
        }
    }
}