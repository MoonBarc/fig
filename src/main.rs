use std::fs::File;

use crate::{
    fe::{ast::print_statements, symbols::SymbolTable, parser::Parser, types, item::Item, scope::Scope},
    be::{
        irgen::IrGen,
        ir::{IrBlock, IrOp, IrOpKind},
        consts::ConstTable,
        platform::arm64::Arm64Generator,
        CompUnit
    }
};

mod fe;
mod be;

fn main() {
    let prog = include_str!("../current.fig");
    let mut syms = SymbolTable::new();
    let consts = ConstTable::new();
    let parser = Parser::new(prog);

    let (mut stmts, errs) = parser.parse();
    if !errs.is_empty() {
        println!("errs: {:?}", errs);
    } else {
        println!("parsing succeeded without errors :)");
    }

    let mut scope = Scope::new();
    scope.resolve_block(&mut syms, &mut stmts);

    types::type_check_block(&mut syms, &mut stmts);
    print_statements(&syms, 0, &stmts);

    let mut block = IrBlock::new();
    let mut generator = IrGen::new();
    println!("{:#?}", &consts);
    let mut comp_unit = CompUnit {
        prog,
        consts,
        items: vec![Item::Function {
            code: stmts,
            return_type: syms.unit()
        }],
    };

    let out = generator.gen(&syms, &mut comp_unit, &mut block);
    
    block.print();
    println!("return val in {:?}", out);
    // return the final value
    // block.ops.push(IrOp {
    //     kind: IrOpKind::Ret(out),
    //     result_into: None
    // });

    println!("generating assembly");
    let mut arm_gen = Arm64Generator::new(
        File::create("./prog_out.s").unwrap(),
        comp_unit
    );
    arm_gen.gen(&mut block);

    println!("o/");
}
