use std::ops::Range;

use super::{token::{Token, CommentType}, Sp};

#[derive(Debug)]
pub enum LexError {
    UnknownCharacter(char),
    NumberParseFailed,
    UnterminatedString(String),
    UnterminatedComment(String)
}

fn is_ident_char(ch: char, start: bool) -> bool {
    (ch.is_alphanumeric() && !(start && is_digit(ch)))
        || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

pub struct Lexer<'a> {
    pub prog: &'a str,
    at: usize,
    at_char: usize,
    line: usize,
    col: usize,
    len: usize,
    semi_insert: bool
}

impl<'a> Lexer<'a> {
    pub fn new(prog: &'a str) -> Lexer<'a> {
        Self {
            prog,
            at: 0,
            at_char: 0,
            line: 1,
            col: 1,
            len: 0,
            semi_insert: false
        }
    }

    pub fn next(&mut self) -> Option<Sp<'a, Token<'a>>> {
        self.skip_whitespace();
        self.semi_insert = false;

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
                    if self.pick('/') {
                        self.comment(3, CommentType::Doc, "\n")
                    } else if self.pick('!') {
                        self.comment(3, CommentType::ModuleDoc, "\n") 
                    } else {
                        self.comment(2, CommentType::Regular, "\n")
                    }
                } else if self.pick('*') {
                    self.comment(2, CommentType::Regular, "*/")
                } else {
                    self.eq_variant(Div, DivEq)
                }
            },
            '%' => self.eq_variant(Mod, ModEq),
            '.' => { 
                if is_digit(self.peek()) {
                    self.number(true)
                } else { Dot }
            }
            '?' => Try,
            '!' => self.eq_variant(Not, NotEq),
            '=' => self.eq_variant(Assign, Eq),
            '<' => self.eq_variant(Lt, LtEq),
            '>' => self.eq_variant(Gt, GtEq),

            '&' if self.pick('&') => And,
            '|' if self.pick('|') => Or,

            '{' => LBrace, '}' => RBrace,
            '(' => LParen, ')' => RParen,
            '[' => LBracket, ']' => RBracket,
            ',' => Comma,
            
            '\n' => Newline,
            
            '@' => At,
            '"' | '\'' => self.string(),
            x if is_ident_char(x, true) => self.identifier(),
            n if is_digit(n) => self.number(false),
            x => Error(LexError::UnknownCharacter(x))
        };
        Some(Sp {
            line: self.line,
            col: self.col,
            of: self.prog, 
            span: self.lexeme_range(),
            data: token
        })
    }

    fn string(&mut self) -> Token<'a> {
        let start_seq = self.lexeme();
        if let Err(e) = self.eat_until(start_seq, LexError::UnterminatedString(start_seq.to_owned())) { return e }
        let total = self.lexeme();
        Token::String(&total[start_seq.len()..total.len() - start_seq.len()])
    }

    fn eat_until(&mut self, end_seq: &str, unterminated_err: LexError) -> Result<(), Token<'a>> {
        loop {
            self.advance();
            let latest_seq = &self.prog.as_bytes()[self.at - end_seq.len()..self.at];
            if latest_seq == end_seq.as_bytes() {
                return Ok(());
            }

            if self.at_end() {
                return Err(Token::Error(unterminated_err));
            }
        }
    }

    fn comment(&mut self, start_len: usize, ty: CommentType, end_seq: &str) -> Token<'a> {
        if let Err(e) = self.eat_until(end_seq, LexError::UnterminatedComment(end_seq.to_owned())) { return e }
        let l = self.lexeme();
        Token::Comment(ty, &l[start_len..l.len()-end_seq.len()])
    }

    fn identifier(&mut self) -> Token<'a> {
        while is_ident_char(self.peek(), false) {
            self.advance();
        }

        let ident = self.lexeme();
        use Token::*;
        match ident {
            "let" => Let,
            "mut" => Mut,
            "if" => If,
            "else" => Else,
            "fn" => Fn,
            "for" => For,
            "while" => While,
            "loop" => Loop,
            "break" => { self.semi_insert = true; Break },
            "continue" => { self.semi_insert = true; Continue },
            "return" => { self.semi_insert = true; Return },
            "enum" => Enum,
            "struct" => Struct,
            "pub" => Pub,
            "import" => Import,
            "match" => Match,
            "default" => Default,

            "true" => { self.semi_insert = true; True },
            "false" => { self.semi_insert = true; False },
            "nil" => { self.semi_insert = true; Nil },

            _ => Identifier(ident)
        }
    }

    // dotted = whether the number started with a dot
    fn number(&mut self, dotted: bool) -> Token<'a> {
        if !dotted {
            while is_digit(self.peek()) { self.advance(); }
        }
        if dotted || self.pick('.') {
            while is_digit(self.peek()) { self.advance(); }
            let Ok(parsed) = self.lexeme().parse::<f64>() else {
                return Token::Error(LexError::NumberParseFailed)
            };
            Token::CompFloat(parsed)
        } else {
            let Ok(parsed) = self.lexeme().parse::<i64>() else {
                return Token::Error(LexError::NumberParseFailed)
            };
            Token::CompInt(parsed)
        }

    }

    fn eq_variant(&mut self, base: Token<'a>, with: Token<'a>) -> Token<'a> {
        self.variant(base, '=', with)
    }

    fn variant(&mut self, base: Token<'a>, with_ch: char, with: Token<'a>) -> Token<'a> {
        if self.pick(with_ch) {
            with
        } else { base }
    }

    fn lexeme(&self) -> &'a str {
        &self.prog[self.lexeme_range()]
    }

    fn lexeme_range(&self) -> Range<usize> {
        self.at - self.len..self.at
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut last_was_newline = false;
        while !self.at_end() {
            match self.peek() {
                ' ' | '\t' => { self.advance(); last_was_newline = true; },
                _ => return last_was_newline
            }
        }
        false
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
        (self.prog[char_start..at].chars().next().unwrap_or('\0'), at - self.at)
    }

    fn peek_to(&self, n: usize) -> char {
        let (r, _) = self.peek_to_with_loc(n);
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

