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
pub const PUSH_TRUE: u8 = 0x06;
pub const PUSH_FALSE: u8 = 0x07;

pub struct VM {
    global: Rc<RefCell<HashMap<String, Value>>>,
    pub consts: Const,
    insts: ByteCode,
    stack: Vec<Value>,
    pc: isize,
    ops: [fn(&mut VM); 8],
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
            ops: [
                end,
                create_context,
                get_global,
                get_member,
                push_const,
                call,
                push_true,
                push_false,
            ],
        }
    }
}

impl VM {
    pub fn run(&mut self, insts: ByteCode) {
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

fn end(_vm: &mut VM) {}

fn create_context(vm: &mut VM) {
    vm.pc += 1;
    get_byte!(vm, _n, usize);
    get_byte!(vm, _argc, usize);
}

fn push_const(vm: &mut VM) {
    vm.pc += 1;
    get_byte!(vm, n, usize);
    vm.stack.push(vm.consts.value[n].clone());
}

fn push_false(self_: &mut VM) {
    self_.pc += 1; 
    self_.stack.push(Value::Bool(false));
}

fn push_true(self_: &mut VM) {
    self_.pc += 1;
    self_.stack.push(Value::Bool(true));
}

fn get_global(vm: &mut VM) {
    vm.pc += 1;
    get_byte!(vm, n, usize);
    let val = (*(*vm.global)
        .borrow()
        .get(vm.consts.string[n].as_str())
        .unwrap())
    .clone();
    vm.stack.push(val);
}

fn get_member(vm: &mut VM) {
    vm.pc += 1;
    let member = vm.stack.pop().unwrap();
    let parent = vm.stack.pop().unwrap();
    if let Value::String(x) = member {
        match parent {
            Value::Object(map) => match map.borrow().get(x.as_str()) {
                Some(addr) => {
                    let val = addr.clone();
                    vm.stack.push(val)
                }
                None => vm.stack.push(Value::Undefined),
            },
            _ => unreachable!(),
        }
    }
}

fn call(vm: &mut VM) {
    vm.pc += 1; // Call
    get_byte!(vm, argc, usize);

    let callee = vm.stack.pop().unwrap();

    loop {
        match callee {
            Value::Console(1) => {
                let mut args = vec![];
                for _ in 0..argc {
                    args.push(vm.stack.pop().unwrap());
                }
                args.reverse();
                console_log(args);
                break;
            }
            c => {
                println!("Call: err: {:?}, pc = {}", c, vm.pc);
                break;
            }
        }
    }

    fn console_log(args: Vec<Value>) {
        println!("{:?}",args);
        let args_len = args.len();
        for i in 0..args_len {
            match args[i] {
                Value::String(ref s) => {
                    println!("{:?}", s);
                }
                Value::Number(ref n) => {
                    println!("{}", n);
                }
                Value::Undefined => {
                    println!("undefined");
                }
                _ => {}
            }
        }
    }
}
