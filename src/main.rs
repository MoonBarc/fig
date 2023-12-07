use std::{io::stdout, fs::File};

use crate::{
    fe::{ast::{print_statements, print_tree},
    symbols::SymbolTable, parser::Parser, types},
    be::{irgen::IrGen, ir::{IrBlock, IrOp, IrOpKind, IrOperand}, consts::ConstTable, platform::arm64::Arm64Generator, CompUnit}
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

    let mut block = IrBlock::new();
    let mut generator = IrGen::new();
    let out = generator.gen_code(&mut consts, &syms, &mut block, ast);
    
    println!("{:#?}", &consts);
    block.print();
    println!("return val in {:?}", out);
    // return the final value
    block.ops.push(IrOp {
        kind: IrOpKind::Ret(out),
        result_into: None
    });

    println!("begin asm:");
    let unit = CompUnit {
        prog: &prog,
        consts,
        items: Vec::new(),
    };
    let mut arm_gen = Arm64Generator::new(
        File::create("./prog_out.s").unwrap(),
        unit
    );
    arm_gen.gen(&block);

    println!("o/");
}
