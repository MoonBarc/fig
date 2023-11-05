use crate::fe::{lexer::Lexer, ast::TypeRegistry};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../example.fig");
    let mut types = TypeRegistry::new();
    let mut lexer = Lexer::new(&prog);

    println!("o/");
}
