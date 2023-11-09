use crate::fe::{ast::{TypeRegistry, print_statements}, parser::Parser};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../import.fig");
    let mut _types = TypeRegistry::new();
    let mut parser = Parser::new(prog);

    let (mut stmts, errs) = parser.parse();
    println!("ast: ");
    print_statements(0, &stmts);
    // print_tree(0, "root", &stmts);
    println!("errs: {:?}", errs);

    println!("o/");
}
