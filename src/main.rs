use crate::fe::{ast::TypeRegistry, parser::Parser};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../example.fig");
    let mut types = TypeRegistry::new();
    let mut parser = Parser::new(prog);

    // let mut ast = parser.parse(); 

    println!("o/");
}
