use super::bytecode::{ByteCode, ByteCodeGen};
use super::id::IdGen;
use super::parser::Node;
use super::vm::Value;

#[derive(Debug, Clone)]
pub struct CodeGen {
    bytecode: ByteCodeGen,
    addr: IdGen,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            bytecode: ByteCodeGen::new(),
            addr: IdGen::new(),
        }
    }
}

impl CodeGen {
    pub fn compile(&mut self, node: &Node, insts: &mut ByteCode) {
        let pos = insts.len();
        self.bytecode.create_context(0, 0, insts);
        self.addr.add();
        self.run(node, insts);
    }

    fn run(&mut self, node: &Node, insts: &mut ByteCode) {
        match node {
            &Node::NodeList(ref node_list) => self.node_list(node_list, insts),
            &Node::Call(ref callee, ref args) => self.call(&*callee, args, insts),
            &Node::Member(ref parent, ref member) => self.member(&*parent, member, insts),
            &Node::Identifier(ref name) => self.identifier(name, insts),
            &Node::Number(n) => self.bytecode.push_const(Value::Number(n), insts),
            _ => {}
        }
    }
}

impl CodeGen {
    pub fn node_list(&mut self, node_list: &Vec<Node>, insts: &mut ByteCode) {
        for node in node_list {
            self.run(node, insts)
        }
    }

    pub fn call(&mut self, callee: &Node, args: &Vec<Node>, insts: &mut ByteCode) {
        for arg in args {
            self.run(arg, insts);
        }

        self.run(callee, insts);

        self.bytecode.call(args.len() as u32, insts);
    }

    fn member(&mut self, parent: &Node, member: &String, insts: &mut ByteCode) {
        self.run(parent, insts);

        self.bytecode.push_const(Value::String(member.clone()), insts);
        self.bytecode.get_member(insts);
    }

    fn identifier(&mut self, name: &String, insts: &mut ByteCode) {
        self.bytecode.get_global(name.clone(), insts);
    }
}
