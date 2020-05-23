use super::token::{Kind, Token};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Lexer {
    code: String,
    pos: usize,
    buf: VecDeque<Token>,
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

impl Lexer {
    pub fn end(&self) -> bool {
        self.pos >= self.code.len()
    }
    pub fn next_char(&self) -> Result<char, ()> {
        self.code[self.pos..].chars().next().ok_or(())
    }

    fn skip_char(&mut self) -> Result<char, ()> {
        let mut iter = self.code[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(())?;
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        Ok(cur_char)
    }
}

impl Lexer {
    pub fn next(&mut self) -> Result<Token, ()> {
        match self.token() {
            Ok(ref tok) if tok.kind == Kind::LineTerminator => self.next(),
            otherwise => otherwise,
        }
    }
    fn token(&mut self) -> Result<Token, ()> {
        if !self.buf.is_empty() {
            return Ok(self.buf.pop_front().unwrap());
        }
        match self.next_char()? {
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '0'..='9' => self.number(),
            _ => self.symbol(),
        }
    }

    fn identifier(&mut self) -> Result<Token, ()> {
        let ident = self.skip_while(|c| c.is_alphanumeric() || c == '_')?;
        Ok(Token::identifier(ident))
    }

    fn skip_while<F>(&mut self, mut f: F) -> Result<String, ()>
    where
        F: FnMut(char) -> bool,
    {
        let mut s = "".to_string();
        while !self.end() && f(self.next_char()?) {
            s.push(self.skip_char()?);
        }
        Ok(s)
    }

    fn number(&mut self) -> Result<Token, ()> {
        let mut last = self.next_char()?;
        let num = self.skip_while(|c| {
            println!("222");
            let is_end_of_num = !c.is_alphanumeric() && c != '.';
            last = c;
            !is_end_of_num
        })?;
        let num: f64 = self.read_num(num.as_str()) as f64
        Ok(Token::number(num))
    }
}
