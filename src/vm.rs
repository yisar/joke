use super::bytecode::ByteCode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    Console(usize),
    Object(Rc<RefCell<HashMap<String, Value>>>),
}

impl Const {
    pub fn new() -> Const {
        Const {
            value: vec![],
            string: vec![],
        }
    }
}

pub const END: u8 = 0x00;
pub const CREATE_CONTEXT: u8 = 0x01;
pub const GET_GLOBAL: u8 = 0x02;
pub const GET_MEMBER: u8 = 0x03;
pub const PUSH_CONST: u8 = 0x04;
pub const CALL: u8 = 0x05;

pub struct VM {
    global: Rc<RefCell<HashMap<String, Value>>>,
    pub consts: Const,
    insts: ByteCode,
    stack: Vec<Value>,
    pc:isize,
    ops: [fn(&mut VM);6]
}

impl VM {
    pub fn new() -> VM {
        let mut map = HashMap::new();
        map.insert("console".to_string(), {
            let mut map = HashMap::new();
            map.insert("log".to_string(), Value::Console(1));
            Value::Object(Rc::new(RefCell::new(map)))
        });
        let global = Rc::new(RefCell::new(map));
        VM {
            global: global.clone(),
            stack: {
                let mut stack = Vec::with_capacity(128);
                stack.push(Value::Object(global.clone()));
                stack
            },
            insts: vec![],
            consts: Const::new(),
            pc: 0isize,
            ops:[
                end,
                create_context,
                get_global,
                get_member,
                push_const,
                call
            ]
        }
    }
}

impl VM {
    pub fn run(&mut self, insts:ByteCode){
        self.insts = insts;
        loop {
            let code = self.insts[self.pc as usize];
            self.ops[code as usize](self);
            if code == END {
                break;
            }
        }
    }
}

macro_rules! get_byte {
    ($self:ident, $var:ident, $ty:ty) => {
        let $var = (($self.insts[$self.pc as usize + 3] as $ty) << 24)
            + (($self.insts[$self.pc as usize + 2] as $ty) << 16)
            + (($self.insts[$self.pc as usize + 1] as $ty) << 8)
            + ($self.insts[$self.pc as usize + 0] as $ty);
        $self.pc += 4;
    };
}

fn end(vm: &mut VM) {}

fn create_context(vm: &mut VM) {
    vm.pc += 1;
    get_byte!(vm, n , usize);
    get_byte!(vm, argc, usize);
}

fn push_const(vm: &mut VM) {
    vm.pc += 1;
    get_byte!(vm, n, usize);
    vm.stack.push(vm.consts.value[n].clone());
}