//! Utilities and algorithms for working with blocks
use std::{mem, rc::Rc};

use super::ir::{IrBlock, IrOpKind};

/// A [CFG](Cfg) node. It can either flow forward or diverge (jump)
/// A leaf will have neither and represents a return (or some other halting)
#[derive(Debug, Default)]
pub struct CfgNode {
    diverge: Option<Rc<CfgNode>>,
    flow: Option<Rc<CfgNode>>
}

impl CfgNode {
    pub fn new() -> Self { Self::default() }
}

/// Control Flow Graph
pub struct Cfg {
    roots: Vec<CfgNode>
}

impl Cfg {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }
}

/// Breaks down a complex block into a series of basic blocks
/// driven by a [Control Flow Graph](Cfg)
pub fn break_down(block: IrBlock) -> (Vec<IrBlock>, Cfg) {
    let mut blocks = vec![];
    // TODO: the actual CFG
    let cfg = Cfg::new();
    let mut starts = Vec::new();
    for (i, instr) in block.ops.iter().enumerate() {
        use IrOpKind::*;
        match instr.kind {
            Jump(n) | JumpEq(n, ..) | JumpNe(n, ..)
            | JumpGz(n, ..) | JumpLz(n, ..) => {
                starts.push(n);
                starts.push(i + 1);
            },
            Ret(..) => { starts.push(i + 1); },
            _ => {}
        }
    }
    let mut diverge: Option<()> = None;
    let mut flow: Option<()> = None;
    let mut b = IrBlock::new();
    for (i, instr) in block.ops.into_iter().enumerate() {
        if starts.contains(&i) {
            // move on to the next block
            let mut new_block = IrBlock::new();
            mem::swap(&mut b, &mut new_block);
            blocks.push(new_block);
            diverge = None;
            flow = None;
        }

        b.ops.push(instr);
    }
    (blocks, cfg)
}
