use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Lexer {
    code: String,
    pos: usize,
    buf: VecDeque<String>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code,
            pos: 0,
            buf: VecDeque::new(),
        }
    }
}
