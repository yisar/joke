use lexer;

#[derive(Debug)]
pub struct Parser {
    pub lexer:lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(code:String) -> Parser{
        Parser{
            lexer:lexer::Lexer::new(code),
        }
    }
}