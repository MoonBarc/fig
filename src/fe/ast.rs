use super::{Sp, symbols::SymbolTable};

#[derive(Debug)]
pub enum CompInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize)
}

#[derive(Debug)]
pub enum CompFloat {
    F32(f32),
    F64(f64)
}

#[derive(Debug)]
pub enum ConstantValue<'a> {
    String(&'a str),
    CompInt(CompInt),
    CompFloat(CompFloat),
    Bool(bool),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Assign,

    Add, AddEq,
    Sub, SubEq,
    Mul, MulEq,
    Div, DivEq,
    Pow, PowEq,
    Mod, ModEq,

    Eq, NotEq,

    Gt, GtEq,
    Lt, LtEq,
    
    And, Or
}

#[derive(Debug, Clone)]
pub enum UnOp {
    // prefix
    Negate,
    Not,
    
    // postfix
    Try
}

#[derive(Debug, Clone)]
pub enum Reference<'a> {
    Unresolved(&'a str),
    Resolved(usize)
}

impl<'a> Reference<'a> {
    pub fn unwrap_str(self) -> &'a str {
        match self {
            Self::Unresolved(a) => a,
            _ => unreachable!()
        }
    }

    pub fn unwrap_resolved(self) -> usize {
        match self {
            Self::Resolved(a) => a,
            _ => panic!("{:?}", self)
            // _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum AstNodeKind<'a> {
    Value(ConstantValue<'a>),
    Reference(Reference<'a>),
    BinOp {
        a: AstNode<'a>,
        b: AstNode<'a>,
        op: Sp<'a, BinOp>
    },
    UnOp {
        op: Sp<'a, UnOp>,
        target: AstNode<'a>
    },
    Error,
    If {
        condition: AstNode<'a>,
        body: AstNode<'a>,
        else_body: Option<AstNode<'a>>
    },
    Loop {
        body: AstNode<'a>
    },
    Block {
        stmts: Vec<Statement<'a>>
    }
}

impl<'a> AstNodeKind<'a> {
    pub fn get_start_end(&self) -> Option<(Sp<'a, ()>, Sp<'a, ()>)> {
        Some(match self {
            Self::BinOp { a, b, .. } => (a.ditch(), b.ditch()),
            Self::UnOp { op, target } => (op.ditch(), target.ditch()),
            _ => return None
        })
    }
}

pub type AstNode<'a> = Sp<'a, RawAstNode<'a>>;

#[derive(Debug)]
pub struct RawAstNode<'a> {
    pub kind: Box<AstNodeKind<'a>>,
    pub type_data: Option<usize>,
}

impl<'a, 'tr> RawAstNode<'a> {
    pub fn new(kind: AstNodeKind<'a>) -> Self {
        Self {
            kind: Box::new(kind),
            type_data: None
        } 
    }
}

#[derive(Debug)]
pub enum ImportElement<'a> {
    Item(&'a str),
    Access(&'a str, Vec<ImportElement<'a>>),
}

#[derive(Debug)]
pub enum MaybeTyped<'a> {
    NotTyped,
    TypeProvided(&'a str),
    TypeResolved(usize)
}

impl<'a> MaybeTyped<'a> {
    pub fn unwrap_type(&self) -> usize {
        match self {
            Self::TypeResolved(r) => return *r,
            _ => panic!("tried to unwrap type but got {:?}", self)
        }
    }

    pub fn unwrap_type_str(self) -> &'a str {
        match self {
            Self::TypeProvided(r) => return r,
            _ => panic!("tried to unwrap type str but got {:?}", self)
        }
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Declare {
        id: Reference<'a>,
        with_type: MaybeTyped<'a>,
        value: AstNode<'a>
    },
    Expression(AstNode<'a>),
    Import {
        paths: Vec<ImportElement<'a>>
    },
    Return(AstNode<'a>),
    Out(AstNode<'a>), // <-
    Continue {
        label: Option<usize>
    },
    Break {
        label: Option<usize>,
        with: Option<AstNode<'a>>
    },
    Error
}

/// The *Beeg* Space String Function®
fn beegstr(len: u16) -> String {
    let mut s = String::with_capacity(len as usize);
    for _ in 0..len {
        s.push(' ');
    }
    s
}

pub fn print_tree(symbols: &SymbolTable, depth: u16, label: &str, node: &AstNode) {
    let s = format!("{}{}: ({:?})", beegstr(depth), label, node.type_data);
    match &*node.kind {
        AstNodeKind::BinOp { a, b, op } => {
            println!("{}BinOp({:?})", s, **op);
            print_tree(symbols, depth + 1, "a", &a);
            print_tree(symbols, depth + 1, "b", &b);
        },
        AstNodeKind::Value(v) => { println!("{}Value({:?})", s, v) },
        AstNodeKind::UnOp { op, target } => {
            println!("{}UnOp({:?})", s, **op);
            print_tree(symbols, depth + 1, "t", &target);
        }
        AstNodeKind::Reference(r) => {
            println!("{}Reference({:?})", s, r);
        },
        AstNodeKind::If { condition, body, else_body } => {
            println!("{}If", s);
            print_tree(symbols, depth + 1, "c", condition);
            print_tree(symbols, depth + 1, "do", body);
            if let Some(eb) = else_body {
                print_tree(symbols, depth + 1, "el", eb);
            }
        },
        AstNodeKind::Block { stmts } => {
            print_statements(symbols, depth + 1, stmts);
        },
        AstNodeKind::Loop { body } => {
            println!("{}Loop", s);
            print_tree(symbols, depth + 1, "body", body);
        }
        AstNodeKind::Error => {
            println!("{}Error", s);
        }
    }
}

pub fn print_statements(symbols: &SymbolTable, depth: u16, stmts: &Vec<Statement>) {
    let s = beegstr(depth);
    for stmt in stmts {
        match stmt {
            Statement::Declare { id, with_type, value } => {
                println!("{}Declare {:?}: {:?}", s, id, with_type);
                print_tree(symbols, depth + 1, "value", &value);
            },
            Statement::Expression(e) => {
                println!("{}Expression", s);
                print_tree(symbols, depth + 1, "e", &e);
            },
            Statement::Return(e) => {
                println!("{}Return", s);
                print_tree(symbols, depth + 1, "r", &e);
            },
            Statement::Out(e) => {
                println!("{}Out", s);
                print_tree(symbols, depth + 1, "<-", &e)
            },
            Statement::Import { paths } => {
                println!("{}Import {:?}", s, paths);
            },
            Statement::Break { label, with } => {
                println!("{}Break({:?})", s, label);
                if let Some(w) = with {
                    print_tree(symbols, depth + 1, "with", &w);
                }
            },
            Statement::Continue { label } => {
                println!("{}Continue({:?})", s, label);
            }
            Statement::Error => {
                println!("{}Error!", s);
            },
        }
    }
}
