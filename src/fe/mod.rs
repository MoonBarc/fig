//! The Frontend
use std::{fmt::Debug, ops::{Deref, DerefMut}};

use self::ast::AstNode;

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;

#[derive(Debug)]
pub struct Sp<'a, T: Debug> {
    pub line: usize,
    pub col: usize,
    pub span: &'a str,
    pub data: T
}

impl<T> Deref for Sp<'_, T> where T: Debug {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Sp<'_, T> where T: Debug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub struct CompileError<'c, 'a, 'tr> {
    // this is the uglest abomination i've ever seen
    span: Sp<'a, &'c AstNode<'a, 'tr>>,
    message: String
}
