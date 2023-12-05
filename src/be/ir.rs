//! HLIR types

#[derive(Debug, Clone)]
pub enum IrOperand {
    /// Looks up a constant by its id from the constant table
    Constant(usize),
    /// Gets/sets the data in the specified variable
    Reference(usize),
    /// Gets/sets the data in a temporary value
    Temporary(usize)
}

#[derive(Debug)]
pub enum IrOpKind {
    /// x = (op1)
    Set(IrOperand),
    /// x = (op1+op2)
    Add(IrOperand, IrOperand),
    /// x = (op1-op2)
    Sub(IrOperand, IrOperand),
    /// x = (op1*op2)
    Mul(IrOperand, IrOperand),
    /// x = (op1/op2)
    Div(IrOperand, IrOperand),
    /// x = (-op1)
    Neg(IrOperand),
    /// () = ret op1
    Ret(IrOperand),
    Jump(usize),
    JumpNe(usize, IrOperand, IrOperand),
    JumpEq(usize, IrOperand, IrOperand),
    JumpLz(usize, IrOperand),
    JumpGz(usize, IrOperand),
}

#[derive(Debug)]
pub struct IrOp {
    pub kind: IrOpKind,
    pub result_into: Option<IrOperand>
}

#[derive(Debug)]
pub struct IrBlock {
    pub ops: Vec<IrOp>
}

impl IrBlock {
    pub fn new() -> Self {
        Self { ops: vec![] }
    }
    
    pub fn print(&self) {
        for instr in &self.ops {
            println!("{} = {:?}", 
                instr.result_into
                    .clone()
                    .map(|f| format!("{:?}", f))
                    .unwrap_or("()".to_string()),
                instr.kind
            )
        }
    }
}
