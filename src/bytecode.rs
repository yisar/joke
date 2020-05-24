use super::vm::{Const, CREATE_CONTEXT};

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

impl ByteCodeGen {
    pub fn create_context(&self, n: usize, argc: usize, insts: &mut ByteCode) {
        insts.push(CREATE_CONTEXT);
        self.gen_int32(n as i32, insts);
        self.gen_int32(argc as i32, insts);
    }
    pub fn gen_int8(&self, n: i8, insts: &mut ByteCode) {
        insts.push(n as u8);
    }

    pub fn gen_int32(&self, n: i32, insts: &mut ByteCode) {
        insts.push(((n >> 0) & 0xff as i32) as u8);
        insts.push(((n >> 8) & 0xff as i32) as u8);
        insts.push(((n >> 16) & 0xff as i32) as u8);
        insts.push(((n >> 24) & 0xff as i32) as u8);
    }

    pub fn replace_int32(&self, n: i32, insts: &mut [u8]) {
        insts[3] = (n >> 24) as u8;
        insts[2] = (n >> 16) as u8;
        insts[1] = (n >> 8) as u8;
        insts[0] = (n >> 0) as u8;
    }
}
