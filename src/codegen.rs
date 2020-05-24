use super::bytecode::{ByteCodeGen,ByteCode};
use super::parser::{Node};

#[derive(Debug, Clone)]
pub struct CodeGen {
    bytecode: ByteCodeGen,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            bytecode: ByteCodeGen::new(),
        }
    }
}

impl CodeGen {
    pub fn compile(&mut self, node: &Node, insts: &mut ByteCode) {
        let pos = insts.len();
        self.bytecode.create_context(0, 0, insts)
    }
}
