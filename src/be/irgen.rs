//! The HLIR (high level intermediate representation) Generator

use std::{collections::HashMap, ops::Range};

use crate::fe::{ast::{AstNode, AstNodeKind, BinOp, UnOp, Statement}, symbols::SymbolTable, item::Item};

use super::{ir::*, consts::ConstTable, CompUnit};

/// generates three address code
pub struct IrGen {
    next_temp: usize
}

impl IrGen {
    pub fn new() -> Self {
        Self {
            next_temp: 0
        }
    }

    pub fn gen<'a>(
        &mut self,
        sym_table: &SymbolTable<'a>,
        unit: &mut CompUnit<'a>,
        target: &mut IrBlock
    ) {
        // TODO: look for main function instead
        let Item::Function { code, .. } = unit.items.swap_remove(0) else { unreachable!() };
        let consts = &mut unit.consts;
        for stmt in code {
            match stmt {
                Statement::Expression(ast) => { self.gen_code(consts, sym_table, target, ast); },
                Statement::Return(ast) => {
                    let out = self.gen_code(consts, sym_table, target, ast);
                    target.ops.push(IrOp {
                        kind: IrOpKind::Ret,
                        ops: vec![out],
                        result_into: None
                    });
                }
                _ => { todo!() }
            };
        }
    }

    /// generates code to evaluate the provided node and returns the ssa id of the result
    pub fn gen_code<'a>(
        &mut self,
        consts: &mut ConstTable<'a>,
        sym_table: &SymbolTable<'a>,
        target: &mut IrBlock,
        ast: AstNode<'a>
    ) -> IrOperand {
        let map = HashMap::<&IrOperand, Range<usize>>::new();
        let n = *ast.data.kind;
        match n {
            AstNodeKind::Value(v) => {
                let constant = consts.add(v);
                let out_id = self.allocate_temp();
                target.ops.push(IrOp {
                    ops: vec![],
                    kind: IrOpKind::LoadC(constant),
                    result_into: Some(out_id.clone())
                });
                out_id
            },
            AstNodeKind::BinOp { a, b, op } => {
                let a = self.gen_code(consts, sym_table, target, a);
                let b = self.gen_code(consts, sym_table, target, b);
                let out_id = self.allocate_temp();

                use IrOpKind::*;
                target.ops.push(IrOp {
                    kind: match *op {
                        BinOp::Add => Add,
                        BinOp::Sub => Sub,
                        BinOp::Mul => Mul,
                        BinOp::Div => Div,
                        _ => todo!("many primitive binops are missing at the moment!")
                    },
                    ops: vec![a, b],
                    result_into: Some(out_id.clone())
                });

                out_id
            },
            AstNodeKind::UnOp { op, target: t } => {
                let target_done = self.gen_code(consts, sym_table, target, t);
                let out_id = self.allocate_temp();

                use IrOpKind::*;
                target.ops.push(IrOp {
                    kind: match *op {
                        UnOp::Negate => Neg,
                        _ => todo!("the other primitive unops aren't working yet!")
                    },
                    ops: vec![target_done],
                    result_into: Some(out_id.clone())
                });

                out_id
            },
            AstNodeKind::Error => panic!("tried to generate code from a faulty AST"),
        }
    }

    fn allocate_temp(&mut self) -> IrOperand {
        IrOperand::Temporary(self.allocate_temp_id())
    }

    fn allocate_temp_id(&mut self) -> usize {
        let i = self.next_temp;
        self.next_temp += 1;
        i
    }
}
