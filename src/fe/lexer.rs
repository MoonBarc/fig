use super::{token::Token, Sp};

#[derive(Debug)]
pub enum LexError {
    UnknownCharacter(char)
}

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
        use Token::*;
        let token = match self.advance() {
            '+' => self.eq_variant(Add, AddEq),
            '-' => {
                if self.pick('>') {
                    Arrow
                } else { self.eq_variant(Sub, SubEq) }
            },
            '*' => {
                if self.pick('*') {
                    self.eq_variant(Pow, PowEq)
                } else {
                    self.eq_variant(Mul, MulEq)
                }
            },
            '/' => {
                if self.pick('/') {
                    // TODO: implement comments
                    Nothing
                } else {
                    self.eq_variant(Div, DivEq)
                }
            },
            '%' => self.eq_variant(Mod, ModEq),
            '.' => Dot,
            '?' => Try,
            '!' => self.eq_variant(Not, NotEq),
            '=' => self.eq_variant(Assign, Eq),
            '<' => self.eq_variant(Lt, LtEq),
            '>' => self.eq_variant(Gt, GtEq),

            '&' if self.pick('&') => And,
            '|' if self.pick('|') => Or,

            x => Error(LexError::UnknownCharacter(x))
        };
        Some(Sp {
            line: self.line,
            col: self.col,
            span: &self.prog[self.at - self.len..self.at],
            data: token
        })
    }

    fn eq_variant(&mut self, base: Token<'a>, with: Token<'a>) -> Token<'a> {
        self.variant(base, '=', with)
    }

    fn variant(&mut self, base: Token<'a>, with_ch: char, with: Token<'a>) -> Token<'a> {
        if self.pick(with_ch) {
            with
        } else { base }
    }

    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            match self.peek() {
                ' ' | '\t' | '\n' => { self.advance(); },
                _ => return
            }
        }
    }

    fn pick(&mut self, ch: char) -> bool {
        if self.peek() == ch {
            self.advance();
            true
        } else { false }
    }

    fn at_end(&self) -> bool {
        self.peek() == '\0' 
    }

    fn peek_to_with_loc(&self, n: usize) -> (char, usize) {
        let mut at = self.at;
        let mut char_start = self.at;
        for _ in 0..=n {
            char_start = at;
            at += 1;
            while !self.prog.is_char_boundary(at.min(self.prog.len())) {
                at += 1;
            }
            at = at.min(self.prog.len());
        }
        (self.prog[char_start..at].chars().next().unwrap_or('\0'), char_start - self.at)
    }

    fn peek_to(&self, n: usize) -> char {
        let (r, _) = self.peek_to_with_loc(n);
        r
    }

    fn peek(&self) -> char {
        self.peek_to(0)
    }

    fn advance(&mut self) -> char {
        let (ch, dist) = self.peek_to_with_loc(1);
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

