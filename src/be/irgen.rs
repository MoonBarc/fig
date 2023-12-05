//! The HLIR (high level intermediate representation) Generator

use crate::fe::{ast::{AstNode, AstNodeKind, BinOp, UnOp}, symbols::SymbolTable};

use super::{ir::*, consts::ConstTable};

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

    /// generates code to evaluate the provided node and returns the ssa id of the result
    pub fn gen_code<'a>(
        &mut self,
        consts: &mut ConstTable<'a>,
        sym_table: &SymbolTable<'a>,
        target: &mut IrBlock,
        ast: AstNode<'a>
    ) -> IrOperand {
        let n = *ast.data.kind;
        match n {
            AstNodeKind::Value(v) => {
                let constant = consts.add(v);
                let out_id = self.allocate_temp();
                target.ops.push(IrOp {
                    kind: IrOpKind::Set(IrOperand::Constant(constant)),
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
                        BinOp::Add => Add(a, b),
                        BinOp::Sub => Sub(a, b),
                        BinOp::Mul => Mul(a, b),
                        BinOp::Div => Div(a, b),
                        _ => todo!("many primitive binops are missing at the moment!")
                    },
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
                        UnOp::Negate => Neg(target_done),
                        _ => todo!("the other primitive unops aren't working yet!")
                    },
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
