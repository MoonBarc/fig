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
                ConstantValue::Bool(b) => {
                    // OPTIMIZATION: This is really low hanging fruit.
                    // A new constant is not necessary for every boolean value, just .true & .false
                    // Maybe a const dedup would fix this
                    
                    // imagine what would happen if someone switched these lol
                    self.instr(&format!(".quad {}", if *b { 1 } else { 0 }))
                },
                ConstantValue::CompFloat(f) => todo!(),
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
                Neg | Not => {
                    let [i] = &instr.ops[..] else { unreachable!() };
                    let x = i.arm_asm();
                    let out = into.unwrap();
                    let iname = match instr.kind {
                        Neg => "neg",
                        Not => "mvn",
                        _ => unreachable!()
                    };
                    self.instr(&format!("{} {}, {}", iname, out, x));
                },
                Cpy => {
                    let [i] = &instr.ops[..] else { unreachable!() };
                    let out = into.unwrap();
                    self.instr(&format!("mov {}, {}", out, i.arm_asm()));
                }
                Ret => {
                    let [i] = &instr.ops[..] else { unreachable!() };
                    let x0 = i.arm_asm();
                    self.instr(&format!("mov x0, {}", x0));
                    self.instr("ret");
                },
                DefMarker(u) => {
                    self.write(&format!("marker_{}:\n", u));
                },
                Jmp(to) => {
                    self.instr(&format!("b marker_{}", to));
                },
                If(yes, no) => {
                    let [cond] = &instr.ops[..] else { unreachable!() };
                    let x = cond.arm_asm();
                    // is it true?
                    self.instr(&format!("cmp {}, #1", x));
                    // yes
                    self.instr(&format!("b.eq marker_{}", yes));
                    // no
                    self.instr(&format!("b marker_{}", no));
                }
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
