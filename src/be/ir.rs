//! SSA IR types

#[derive(Debug)]
pub enum IrOperand {
    /// Looks up a constant by its id from the constant table
    Constant(usize),
    /// Gets the data in the specified variable
    Reference(usize)
}

#[derive(Debug)]
pub enum IrOpKind {
    Add(IrOperand, IrOperand),
    Ret(IrOperand),
}

#[derive(Debug)]
pub struct IrOp {
    kind: IrOpKind,
    result_into: Option<usize>
}

#[derive(Debug)]
pub struct IrBlock {
    ops: Vec<IrOp>
}

impl IrBlock {
    pub fn new() -> Self {
        Self { ops: vec![] }
    }
}

