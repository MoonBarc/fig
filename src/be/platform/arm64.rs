use std::{io::{Write, BufWriter}, collections::HashMap};

use crate::{be::{ir::{IrBlock, IrOpKind, IrOperand, IrOp}, consts::ConstTable, CompUnit}, fe::ast::{ConstantValue, CompInt}};

#[derive(Debug)]
pub enum Register {
    /// represents arm x{n} register
    X(usize),
    /// it spilled to ram
    Spilled(usize)
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum VarTy {
    // a variable from the symtable
    Var(usize),
    // a temporary of this block
    Temporary(usize),
    // a code-level temporary that expires immediately
    CLTemp
}

#[derive(Debug)]
pub struct RegisterAllocation {
    reg: usize,
    // pc where this variable dies
    until: usize
}

pub struct Arm64Generator<'a, T: Write> {
    next_block_id: usize,
    reg_alloc: HashMap<VarTy, RegisterAllocation>,
    output: BufWriter<T>,
    unit: CompUnit<'a>
}

impl<'a, T: Write> Arm64Generator<'a, T> {
    pub fn new(wr: T, unit: CompUnit<'a>) -> Self {
        let mut s = Self {
            next_block_id: 0,
            output: BufWriter::new(wr),
            reg_alloc: HashMap::new(),
            unit
        };
        s.generate_const_block();
        s.header();
        s
    }

    fn generate_const_block(&mut self) {
        self.write(".data:\n");
        let nconsts = self.unit.consts.consts.len();
        let mut names = Vec::with_capacity(nconsts);

        for i in 0..nconsts {
            names.push(format!("c{}", i));
            self.write(&format!("{}:\n", names[i]));
            match &self.unit.consts.consts[i] {
                ConstantValue::String(str) => {
                    // HACK: the debug representation *should* escape stuff
                    // for me but this could be a bad idea
                    self.instr(&format!(".ascii {:?}", str));
                },
                ConstantValue::CompInt(i) => {
                    match i {
                        CompInt::I64(i) => {
                            self.instr(&format!(".quad {}", (*i) as usize));
                        },
                        _ => todo!("other number types are not supported yet!")
                    } 
                },
                ConstantValue::CompFloat(f) => todo!(),
                ConstantValue::Bool(_) => todo!(),
                ConstantValue::Nil => todo!(),
            }
        }

        self.unit.consts.const_names = names;

        // we're done, move on to the program part
        self.write(".text:\n");
    }

    fn get_reg_for_op(&self, op: &IrOperand) -> usize {
        match op {
            IrOperand::Constant(_) => panic!("can't get a register for a constant"),
            IrOperand::Reference(..) | IrOperand::Temporary(..) => {
                let varty = match op {
                    IrOperand::Reference(r) => VarTy::Var(*r),
                    IrOperand::Temporary(t) => VarTy::Temporary(*t),
                    _ => unreachable!()
                };
                self.reg_alloc.get(&varty).unwrap().reg
            },
        }
    }

    fn get_temporary_reg(&mut self) -> usize {
        let i = self.reg_alloc.len();
        self.reg_alloc.insert(VarTy::CLTemp, RegisterAllocation {
            reg: i,
            until: 0    
        });
        i
    }

    pub fn gen(&mut self, entry: &IrBlock) {
        // HACK: stupidest possible register allocation incoming!
        
        let mut i = 0;
        for instr in &entry.ops {
            let Some(reg) = instr.result_into.clone() else { continue };
            self.reg_alloc.insert(match reg {
                IrOperand::Reference(r) => VarTy::Var(r),
                IrOperand::Temporary(d) => VarTy::Temporary(d),
                _ => panic!("can't store anything in a constant!")
            }, RegisterAllocation {
                reg: i,
                until: 489048,
            });
            i += 1;
        }
        
        use IrOpKind::*;

        for instr in &entry.ops {
            match &instr.kind {
                Set(to_load) => {
                    let into = instr.result_into.clone().unwrap();
                    let into = format!("x{}", self.get_reg_for_op(&into));
                    let l = match to_load {
                        IrOperand::Constant(c) => {
                            format!("ldr {}, {}", into, self.unit.consts.const_names[*c])
                        },
                        _ => {
                            format!("mov {}, {}", into, self.get_reg_for_op(to_load))
                        },
                    };
                    self.instr(&l);
                },
                Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
                    let out = self.get_reg_for_op(&instr.result_into.clone().unwrap());
                    let a = self.get_reg_for_op(a);
                    let b = self.get_reg_for_op(b);
                    let iname = match instr.kind {
                        Add(..) => "add",
                        Sub(..) => "sub",
                        Mul(..) => "mul",
                        Div(..) => "sdiv",
                        _ => unreachable!()
                    };
                    self.instr(&format!("{} x{}, x{}, x{}", iname, out, a, b));
                },
                Neg(i) => {
                    let x = self.get_reg_for_op(i);
                    let out = self.get_reg_for_op(&instr.result_into.clone().unwrap());
                    self.instr(&format!("neg x{}, x{}", out, x));
                }
                Ret(i) => {
                    let x0 = self.get_reg_for_op(i);
                    self.instr(&format!("mov x0, x{}", x0));
                    self.instr("ret");
                },
                _ => {}
            }
        }
    }

    fn instr(&mut self, t: &str) {
        self.write(&format!("    {}\n", t));
    }

    fn header(&mut self) {
        self.write(
            ".global _fig_entrypoint\n_fig_entrypoint:\n"
        );
    }

    fn block(&mut self) -> usize {
        let b = self.next_block_id;
        self.write(&format!("_{}:", b));
        self.next_block_id += 1;
        b
    }

    fn write(&mut self, txt: &str) {
        write!(self.output, "{}", txt)
            .expect("failed to write to assembly target");
    }
}
