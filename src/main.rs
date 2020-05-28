extern crate clap;

use clap::{App, Arg};
use joke::codegen;
use joke::parser;
use joke::vm;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let app = App::new("joke").arg(Arg::with_name("file").index(1));
    let app_matches = app.clone().get_matches();
    if let Some(filename) = app_matches.value_of("file") {
        run(filename);
    }
}

fn run(name: &str) {
    let mut body = String::new();
    match OpenOptions::new().read(true).open(name) {
        Ok(mut ok) => ok.read_to_string(&mut body).ok().expect("cannot read file"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("[Code] {:?}", body);

    let mut parser = parser::Parser::new(body);

    let mut nodes = vec![];
    while let Ok(ok) = parser.next() {
        nodes.push(ok)
    }

    println!("[NodeList] {:?}", nodes);

    let mut codegen = codegen::CodeGen::new();
    let mut insts = vec![];
    codegen.compile(&nodes[0], &mut insts);

    println!("[ByteCode] {:?}", insts);

    let mut vm = vm::VM::new();
    vm.consts = codegen.bytecode.consts;

    println!("Result:");

    vm.run(insts);
}
