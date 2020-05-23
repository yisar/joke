use super::lexer;

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexer: lexer::Lexer,
}

pub enum Node {
    StateList(Vec<Node>),
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

        Ok(Node::StateList(items))
    }

    fn read_atom(&mut self) -> Result<Node, ()> {
        let tok = self.lexer.next()?;
        match tok.kind {
            _ => {
                self.lexer.unget(&tok);
                self.read_expression_statement()
            }
        }
    }
}
