//! The IR Generator

use crate::fe::{ast::{AstNode, AstNodeKind}, symbols::SymbolTable};

use super::{ir::*, consts::ConstsTable};

// generates SSA IR
pub struct IrGen {
    ssa_next_id: usize
}

impl IrGen {
    pub fn new() -> Self {
        Self {
            ssa_next_id: 0
        }
    }

    /// generates code to evaluate the provided node and returns the ssa id of the result
    pub fn gen_code<'a>(
        &mut self,
        consts: &mut ConstsTable<'a>,
        sym_table: &SymbolTable<'a>,
        target: &mut IrBlock,
        ast: AstNode<'a>
    ) -> usize {
        let n = *ast.data.kind;
        match n {
            AstNodeKind::Value(v) => {
                let constant = consts.add(v);
                let out_id = self.allocate_id();
                target.ops.push(IrOp {
                    kind: IrOpKind::Set(IrOperand::Constant(constant)),
                    result_into: Some(out_id)
                });
                out_id
            },
            AstNodeKind::BinOp { a, b, op } => todo!(),
            AstNodeKind::UnOp { op, target } => todo!(),
            AstNodeKind::Error => todo!(),
        }
    }

    fn allocate_id(&mut self) -> usize {
        let i = self.ssa_next_id;
        self.ssa_next_id += 1;
        i
    }
}
