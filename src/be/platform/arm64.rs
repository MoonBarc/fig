use std::io::{Write, BufWriter};

use crate::{
    be::{ir::{IrBlock, IrOpKind, IrOperand}, CompUnit, platform::ra_profile::ArmRegAlloc, ralloc::RegAllocProfile},
    fe::ast::{ConstantValue, CompInt}
};

trait IntoArmReg {
    fn arm_asm(&self) -> String;
}

impl IntoArmReg for IrOperand {
    fn arm_asm(&self) -> String {
        let n = self.unwrap_reg();
        format!("x{}", n)
    }
}

pub struct Arm64Generator<'a, T: Write> {
    next_block_id: usize,
    output: BufWriter<T>,
    unit: CompUnit<'a>
}

impl<'a, T: Write> Arm64Generator<'a, T> {
    pub fn new(wr: T, unit: CompUnit<'a>) -> Self {
        let mut s = Self {
            next_block_id: 0,
            output: BufWriter::new(wr),
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

    pub fn gen(&mut self, entry: &mut IrBlock) {
        use IrOpKind::*;

        let ra = ArmRegAlloc::make();

        ra.allocate_for(entry);

        for instr in &entry.ops {
            let into = instr.result_into
                .clone()
                .map(|r| r.arm_asm());
            match &instr.kind {
                LoadC(c) => {
                    let into = into.unwrap();
                    let t = format!("ldr {}, {}", into, self.unit.consts.const_names[*c]);
                    self.instr(&t);
                },
                Add | Sub | Mul | Div => {
                    let out = into.unwrap();
                    let [a, b] = &instr.ops[..] else { unreachable!() };
                    let a = a.arm_asm();
                    let b = b.arm_asm();
                    let iname = match instr.kind {
                        Add => "add",
                        Sub => "sub",
                        Mul => "mul",
                        Div => "sdiv",
                        _ => unreachable!()
                    };
                    self.instr(&format!("{} {}, {}, {}", iname, out, a, b));
                },
                Neg => {
                    let [i] = &instr.ops[..] else { unreachable!() };
                    let x = i.arm_asm();
                    let out = into.unwrap();
                    self.instr(&format!("neg {}, {}", out, x));
                }
                Ret => {
                    let [i] = &instr.ops[..] else { unreachable!() };
                    let x0 = i.arm_asm();
                    self.instr(&format!("mov x0, {}", x0));
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
