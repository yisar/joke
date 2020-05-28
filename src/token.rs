#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    OpeningParen,
    ClosingParen,
    Point,
    Hash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Identifier(String),
    Number(f64),
    String(String),
    Symbol(Symbol),
    Bool(bool),
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

    pub fn symbol(symbol: Symbol) -> Token {
        Token {
            kind: Kind::Symbol(symbol),
        }
    }

    pub fn string(s: String) -> Token {
        Token {
            kind: Kind::String(s),
        }
    }

    pub fn line_terminator() -> Token {
        Token {
            kind: Kind::LineTerminator,
        }
    }
}
