//! The Frontend
use std::{fmt::Debug, ops::{Deref, DerefMut}};

pub mod token;
pub mod lexer;
pub mod ast;

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
