use lexer;

#[derive(Debug)]
pub struct Parser {
    pub lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(code: String) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(code),
        }
    }
    pub fn next(&mut self) -> Result<None,()>{
        self.read_script()
    }
}

impl Parser {
    pub fn read_script(&mut self) -> Result<None,()>{
        let mut items = vec![];

        loop {
            unimplemented!();
        }
    }
}