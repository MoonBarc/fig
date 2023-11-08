//! The Frontend
use std::{fmt::Debug, ops::{Deref, DerefMut, Range}};

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;

#[derive(Debug)]
pub struct Sp<'a, T: Debug> {
    pub line: usize,
    pub col: usize,
    pub span: Range<usize>,
    pub of: &'a str,
    pub data: T
}

impl<'a, T: Debug> Sp<'a, T> {
    pub fn map<O: Debug>(&self, map_fn: impl FnOnce(&T) -> O) -> Sp<'a, O> {
        let d = map_fn(&self.data);
        Sp {
            data: d,
            line: self.line,
            col: self.col,
            span: self.span.clone(),
            of: self.of,
        }
    }

    pub fn ditch(&self) -> Sp<'a, ()> {
        self.map(|_| ())
    }
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

#[derive(Debug)]
pub struct CompileError<'a> {
    span: &'a str,
    message: String
}
