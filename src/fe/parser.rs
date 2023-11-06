use std::mem;

use super::{token::Token, Sp, lexer::Lexer, ast::{AstNode, Statement}, CompileError};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Sp<'a, Token<'a>>,
    next: Sp<'a, Token<'a>>,
    panicking: bool,
    errors: Vec<CompileError<'a, 'a, 'static>>
}

impl<'a> Parser<'a> {
    pub fn new(prog: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(prog);
        let first = lexer.next();
        Self {
            next: first.unwrap(),
            current: Token::nothing_span(),
            lexer,
            errors: Vec::new(),
            panicking: false
        } 
    }

    // pub fn parse(self) -> Vec<Statement> {
    //     
    // }
    //
    // pub fn parse_expr(&mut self) -> AstNode<'a, 'static> {
    //     
    // }

    // -- Utility Functions --

    pub fn advance(&mut self) -> &Token<'a> {
        let next = self.lexer.next().unwrap_or_else(|| Token::nothing_span());
        mem::swap(&mut self.next, &mut self.current);
        self.next = next;
        &self.current
    }

    pub fn pick(&mut self, tt: Token) -> bool {
        if mem::discriminant(&tt) == mem::discriminant(&self.next) {
            self.advance();
            true
        } else { false }
    }
}
