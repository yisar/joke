#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
}

#[derive(Debug, Clone)]
pub enum Kind {
    Identifier(String),
    Number(f64),
    LineTerminator,
}

impl Token {
    pub fn identifier(ident: String) -> Token {
        Token {
            kind: Kind::Identifier(ident),
        }
    }
    pub fn number(num: f64) -> Token {
        Token {
            kind: Kind::Number(num),
        }
    }
}