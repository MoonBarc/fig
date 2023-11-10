use std::collections::HashMap;

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

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum AstNodeKind<'a> {
    Value(ConstantValue<'a>),
    BinOp {
        a: AstNode<'a>,
        b: AstNode<'a>,
        op: Sp<'a, BinOp>
    },
    UnOp {
        op: Sp<'a, UnOp>,
        target: AstNode<'a>
    },
    Error
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
    pub fn unwrap_type(self) -> usize {
        match self {
            Self::TypeResolved(r) => return r,
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
        id: &'a str,
        with_type: MaybeTyped<'a>,
        value: AstNode<'a>
    },
    Expression(AstNode<'a>),
    Import {
        paths: Vec<ImportElement<'a>>
    }
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
        _ => println!("unknown")
    }
}

pub fn print_statements(symbols: &SymbolTable, depth: u16, stmts: &Vec<Statement>) {
    let s = beegstr(depth);
    for stmt in stmts {
        match stmt {
            Statement::Declare { id, with_type, value } => {
                println!("{}Declare {}: {:?}", s, id, with_type);
                print_tree(symbols, depth + 1, "value", &value);
            },
            Statement::Expression(e) => {
                println!("{}Expression", s);
                print_tree(symbols, depth + 1, "e", &e);
            },
            Statement::Import { paths } => {
                println!("{}Import {:?}", s, paths);
            },
        }
    }
}
