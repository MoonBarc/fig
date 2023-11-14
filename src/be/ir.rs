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
    /// x = (op1)
    Set(IrOperand),
    /// x = (op1+op2)
    Add(IrOperand, IrOperand),
    /// () = ret op1
    Ret(IrOperand),
}

#[derive(Debug)]
pub struct IrOp {
    pub kind: IrOpKind,
    pub result_into: Option<usize>
}

#[derive(Debug)]
pub struct IrBlock {
    pub ops: Vec<IrOp>
}

impl IrBlock {
    pub fn new() -> Self {
        Self { ops: vec![] }
    }
}

