use super::{token::Token, Sp};

pub struct Lexer<'a> {
    prog: &'a str,
    at: usize,
    at_char: usize,
    line: usize,
    col: usize,
    len: usize
}

impl<'a> Lexer<'a> {
    pub fn new(prog: &'a str) -> Lexer<'a> {
        Self {
            prog,
            at: 0,
            at_char: 0,
            line: 1,
            col: 1,
            len: 0
        }
    }

    pub fn next(&mut self) -> Option<Sp<Token<'a>>> {
        self.skip_whitespace();
        self.len = 0;
        if self.at_end() { return None }
        self.advance();
        Some(Sp {
            line: self.line,
            col: self.col,
            span: &self.prog[self.at - self.len..self.at],
            data: Token::Eq
        })
    }

    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            match self.peek() {
                ' ' | '\t' | '\n' => { self.advance(); },
                _ => return
            }
        }
    }

    fn at_end(&self) -> bool {
        self.peek() == '\0' 
    }

    fn peek_to_with_loc(&self, n: usize) -> (char, usize) {
        let mut at = self.at + 1;
        let mut char_start = self.at;
        for _ in 0..=n {
            char_start = at - 1;
            while !self.prog.is_char_boundary(at.min(self.prog.len())) {
                at += 1;
            }
            at = at.min(self.prog.len());
        }
        (self.prog[char_start..at].chars().next().unwrap_or('\0'), at - char_start)
    }

    fn peek_to(&self, n: usize) -> char {
        let (r, x) = self.peek_to_with_loc(n);
        r
    }

    fn peek(&self) -> char {
        self.peek_to(0)
    }

    fn advance(&mut self) -> char {
        let (ch, dist) = self.peek_to_with_loc(0);
        self.at += dist;
        self.at_char += 1;
        self.col += 1;
        self.len += dist;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        }
        ch    
    }
}


