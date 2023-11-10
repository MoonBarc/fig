//! The IR Generator

use crate::fe::{ast::AstNode, symbols::SymbolTable};

use super::ir::*;

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
        sym_table: &SymbolTable<'a>,
        target: &mut IrBlock,
        ast: &AstNode<'a>
    ) -> usize {
         self.allocate_id()
    }

    fn allocate_id(&mut self) -> usize {
        let i = self.ssa_next_id;
        self.ssa_next_id += 1;
        i
    }
}
