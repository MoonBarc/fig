use std::io::{Write, BufWriter};

use crate::be::{ir::{IrBlock, IrOpKind, IrOperand}, consts::ConstTable, CompUnit};

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
        s.header();
        s
    }

    pub fn gen(&mut self, entry: &IrBlock) {
        for instr in &entry.ops {
            match &instr.kind {
                IrOpKind::Set(to_load) => {
                    match to_load {
                        IrOperand::Constant(c) => todo!(),
                        IrOperand::Reference(_) => todo!(),
                        IrOperand::Temporary(_) => todo!(),
                    }
                },
                IrOpKind::Add(_, _) => todo!(),
                IrOpKind::Ret(_) => todo!(),
                _ => todo!()
            }
        }
    }

    fn instr(&mut self, t: &str) {
        self.write(&format!("    {}", t));
    }

    fn header(&mut self) {
        self.write(
            "global _fig_entrypoint\n_fig_entrypoint:"
        );
    }

    fn block(&mut self) -> usize {
        let b = self.next_block_id;
        self.write(&format!("_{}:", b));
        self.next_block_id += 1;
        b
    }

    fn debug_print_result(&mut self) {
        
    }

    fn write(&mut self, txt: &str) {
        write!(self.output, "{}", txt)
            .expect("failed to write to assembly target");
    }
}
