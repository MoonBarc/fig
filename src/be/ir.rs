//! HLIR types

pub type Register = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrOperand {
    /// Gets/sets the data in the specified variable
    Reference(usize),
    /// Gets/sets the data in a temporary value
    Temporary(usize),
    /// Represents a raw, system register.
    /// Only kind available after graph coloring
    /// 0 = eax | x0 depending on the platform
    Register(Register),
    Spilled(usize),
}

impl IrOperand {
    pub fn unwrap_reg(&self) -> Register {
        match self {
            Self::Register(r) => *r,
            _ => panic!("tried to unwrap register but it was {:?}", self)
        }
    }
}

#[derive(Debug)]
pub enum IrOpKind {
    /// x = (CONST)
    LoadC(usize),
    /// x = (op1)
    Cpy,
    /// x = (op1+op2)
    Add,
    /// x = (op1-op2)
    Sub,
    /// x = (op1*op2)
    Mul,
    /// x = (op1/op2)
    Div,
    /// x = (op1 == op2)
    Eq,
    /// x = (op1 != op2)
    NotEq,
    /// x = (-op1)
    Neg,
    /// x = (!op1)
    Not,
    /// () = ret op1
    Ret,
    
    /// (emits marker #(id))
    DefMarker(usize),
    /// jmp -> marker #(id)
    Jmp(usize),
    /// if (op1) then jmp marker #(id) else jmp marker #(eid)
    If(usize, usize)
}

#[derive(Debug)]
pub struct IrOp {
    pub kind: IrOpKind,
    pub ops: Vec<IrOperand>,
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
            println!("{} = {:?} {}",
                instr.result_into
                    .clone()
                    .map(|f| format!("{:?}", f))
                    .unwrap_or("()".to_string()),
                instr.kind,
                instr.ops
                    .iter()
                    .map(|f| format!("{:?}", f))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}
