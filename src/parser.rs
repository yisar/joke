use super::lexer;
use super::token::{Kind, Symbol};

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexer: lexer::Lexer,
}

#[derive(Debug, Clone)]
pub enum Node {
    NodeList(Vec<Node>),
    Call(Box<Node>, Vec<Node>),
    Member(Box<Node>, String),
    Identifier(String),
    Number(f64),
    Node,
}

impl Parser {
    pub fn new(code: String) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(code),
        }
    }
}

impl Parser {
    pub fn next(&mut self) -> Result<Node, ()> {
        self.read_script()
    }
}

impl Parser {
    fn read_script(&mut self) -> Result<Node, ()> {
        let mut items = vec![];
        loop {
            if self.lexer.end() {
                if items.is_empty() {
                    return Err(());
                }
                break;
            }
            if let Ok(item) = self.read_atom() {
                items.push(item)
            }
        }

        Ok(Node::NodeList(items))
    }

    fn read_atom(&mut self) -> Result<Node, ()> {
        self.call()
    }
}

impl Parser {
    fn call(&mut self) -> Result<Node, ()> {
        let mut lhs = self.begin()?;
        while let Ok(tok) = self.lexer.next() {
            match tok.kind {
                Kind::Symbol(Symbol::OpeningParen) => {
                    let args = self.args()?;
                    lhs = Node::Call(Box::new(lhs), args);
                }
                Kind::Symbol(Symbol::Point) => match self.lexer.next()?.kind {
                    Kind::Identifier(name) => lhs = Node::Member(Box::new(lhs), name),
                    _ => {}
                },
                _ => {
                    self.lexer.unget(&tok);
                    break;
                }
            }
        }
        Ok(lhs)
    }

    fn begin(&mut self) -> Result<Node, ()> {
        let tok = self.lexer.next()?;
        match tok.kind {
            Kind::Identifier(id) => Ok(Node::Identifier(id)),
            Kind::Number(num) => Ok(Node::Number(num)),
            Kind::LineTerminator => self.begin(),
            e => unimplemented!("{:?}", e),
        }
    }

    fn args(&mut self) -> Result<Vec<Node>, ()> {
        let tok = self.lexer.next()?;
        match tok.kind {
            Kind::Symbol(Symbol::ClosingParen) => return Ok(vec![]),
            _ => {
                self.lexer.unget(&tok);
            }
        }

        let mut args = vec![];
        loop {
            match self.lexer.next() {
                Ok(ref tok) if tok.kind == Kind::Symbol(Symbol::ClosingParen) => break,
                Ok(tok) => self.lexer.unget(&tok),
                Err(_) => break,
            }

            if let Ok(arg) = self.call() {
                args.push(arg)
            }
        }

        Ok(args)
    }
}
