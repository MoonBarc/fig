use crate::{
    fe::{ast::{print_statements, print_tree},
    symbols::SymbolTable, parser::Parser, types},
    be::{irgen::IrGen, ir::IrBlock, consts::ConstTable}
};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../expr.fig");
    let mut syms = SymbolTable::new();
    let mut consts = ConstTable::new();
    let mut parser = Parser::new(prog);

    let (mut ast, errs) = parser.parse_expr();
    types::type_check(&syms, &mut ast);
    println!("ast: ");
    // print_statements(&syms, 0, &stmts);
    print_tree(&syms, 0, "root", &ast);
    println!("errs: {:?}", errs);

    let mut generator = IrGen::new();
    let mut block = IrBlock::new();
    let out = generator.gen_code(&mut consts, &syms, &mut block, ast);
    println!("{:#?}", &consts);
    block.print();
    println!("return val in {:?}", out);

    println!("o/");
}
