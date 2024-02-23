//! The IR Generator

use crate::fe::{ast::{AstNode, AstNodeKind, BinOp, UnOp, Statement}, symbols::SymbolTable, item::Item};

use super::{ir::*, consts::ConstTable, CompUnit};

pub struct IrContext {
    break_to: Option<usize>,
    break_out: Option<IrOperand>,
    continue_to: Option<usize>
}

/// generates three address code
pub struct IrGen {
    next_temp: usize,
    next_marker: usize,
    context: Vec<IrContext>
}

impl IrGen {
    pub fn new() -> Self {
        Self {
            next_temp: 0,
            next_marker: 0,
            context: vec![]
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
        let out = &self.allocate_temp();
        self.gen_block_code(consts, sym_table, target, code, out);
    }

    fn gen_block_code<'a>(
        &mut self,
        consts: &mut ConstTable<'a>,
        sym_table: &SymbolTable<'a>,
        target: &mut IrBlock,
        stmts: Vec<Statement<'a>>,
        uni_out: &IrOperand
    ) {
        let exit = self.allocate_new_marker();
        for stmt in stmts {
            match stmt {
                Statement::Expression(ast) => { self.gen_code(consts, sym_table, target, ast); },
                Statement::Return(ast) => {
                    let out = self.gen_code(consts, sym_table, target, ast);
                    target.ops.push(IrOp {
                        kind: IrOpKind::Ret,
                        ops: vec![out],
                        result_into: None
                    });
                },
                Statement::Declare {
                    id,
                    value,
                    ..
                } => {
                    let out = self.gen_code(consts, sym_table, target, value);
                    target.ops.push(IrOp {
                        kind: IrOpKind::Cpy,
                        ops: vec![out],
                        result_into: Some(IrOperand::Reference(id.unwrap_resolved()))
                    });
                },
                Statement::Out(val) => {
                    let out = self.gen_code(consts, sym_table, target, val);
                    target.ops.push(IrOp {
                        kind: IrOpKind::Cpy,
                        ops: vec![out],
                        result_into: Some(uni_out.clone())
                    });
                    target.ops.push(IrOp {
                        kind: IrOpKind::Jmp(exit),
                        ops: vec![],
                        result_into: None
                    })
                },
                Statement::Continue { label } => {
                    if label.is_some() {
                        todo!("continue labels");
                    }
                    let ctx = self.context.last().unwrap();
                    target.ops.push(IrOp {
                        kind: IrOpKind::Jmp(ctx.continue_to.unwrap()),
                        ops: vec![],
                        result_into: None
                    })
                }
                Statement::Break { label, with } => {
                    if label.is_some() {
                        todo!("break labels")
                    }
                    let ctx = self.context.last().unwrap();
                    let break_to = ctx.break_to.unwrap();
                    
                    if let Some(with) = with {
                        let break_out = ctx.break_out.clone();
                        let out = self.gen_code(consts, sym_table, target, with);

                        target.ops.push(IrOp {
                            kind: IrOpKind::Cpy,
                            ops: vec![out],
                            result_into: break_out
                        })
                    }

                    target.ops.push(IrOp {
                        kind: IrOpKind::Jmp(break_to),
                        ops: vec![],
                        result_into: None
                    });
                }
                _ => { todo!() }
            };
        };
        self.push_marker(target, exit);
    }

    /// generates code to evaluate the provided node and returns the ir operand of the result
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

                // special case
                if *op == BinOp::Assign {
                    target.ops.push(IrOp {
                        kind: Cpy,
                        ops: vec![b.clone()],
                        result_into: Some(a.clone())
                    });
                    return a;
                }

                use IrOpKind::*;
                target.ops.push(IrOp {
                    kind: match *op {
                        BinOp::Add => Add,
                        BinOp::Sub => Sub,
                        BinOp::Mul => Mul,
                        BinOp::Div => Div,
                        BinOp::Eq => Eq,
                        BinOp::NotEq => NotEq,
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
                        UnOp::Not => Not,
                        _ => todo!("the other primitive unops aren't working yet!")
                    },
                    ops: vec![target_done],
                    result_into: Some(out_id.clone())
                });

                out_id
            },
            AstNodeKind::Reference(r) => {
                IrOperand::Reference(r.unwrap_resolved())
            }
            AstNodeKind::If { condition, body, else_body } => {
                let cond = self.gen_code(consts, sym_table, target, condition);

                let uout = self.allocate_temp();

                let true_branch = self.allocate_new_marker();
                let false_branch = self.allocate_new_marker();
                let bypass = self.allocate_new_marker();

                target.ops.push(IrOp {
                    kind: IrOpKind::If(true_branch, false_branch),
                    ops: vec![cond],
                    result_into: Some(uout.clone())
                });

                self.push_marker(target, true_branch);
                let local_out = self.gen_code(consts, sym_table, target, body);
                target.ops.push(IrOp {
                    kind: IrOpKind::Cpy,
                    ops: vec![local_out.clone()],
                    result_into: Some(uout.clone())
                });
                target.ops.push(IrOp {
                    kind: IrOpKind::Jmp(bypass),
                    ops: vec![],
                    result_into: None
                });
                
                // this is purposefully outside of the else check.
                // if there is no else code, this just points to the end.
                self.push_marker(target, false_branch);
                if let Some(eb) = else_body {
                    let local_out = self.gen_code(consts, sym_table, target, eb);
                    target.ops.push(IrOp {
                        kind: IrOpKind::Cpy,
                        ops: vec![local_out.clone()],
                        result_into: Some(uout.clone())
                    });
                }

                self.push_marker(target, bypass);

                uout
            },
            AstNodeKind::Block { stmts } => {
                let o = self.allocate_temp();
                self.gen_block_code(consts, sym_table, target, stmts, &o);
                o
            },
            AstNodeKind::Loop { body } => {
                let start_mark = self.push_new_marker(target);
                let end_mark = self.allocate_new_marker();
                let o = self.allocate_temp();
                self.context.push(IrContext {
                    continue_to: Some(start_mark),
                    break_out: Some(o.clone()),
                    break_to: Some(end_mark)
                });
                self.gen_code(consts, sym_table, target, body);
                target.ops.push(IrOp {
                    kind: IrOpKind::Jmp(start_mark),
                    ops: vec![],
                    result_into: None
                });
                self.push_marker(target, end_mark);
                self.context.pop();
                o
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

    fn allocate_new_marker(&mut self) -> usize {
        let i = self.next_marker;
        self.next_marker += 1;
        i
    }

    fn push_new_marker(&mut self, target: &mut IrBlock) -> usize {
        let i = self.allocate_new_marker();
        self.push_marker(target, i);
        i
    }

    fn push_marker(&mut self, target: &mut IrBlock, marker: usize) {
        target.ops.push(IrOp {
            kind: IrOpKind::DefMarker(marker),
            ops: vec![],
            result_into: None
        });
    }
}
