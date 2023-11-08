use crate::fe::{ast::{TypeRegistry, print_tree}, parser::Parser};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../expr.fig");
    let mut _types = TypeRegistry::new();
    let mut parser = Parser::new(prog);

    let (mut ast, errs) = parser.parse_expr();
    println!("ast:");
    print_tree(0, "root", &ast);
    println!("errs: {:?}", errs);

    println!("o/");
}
