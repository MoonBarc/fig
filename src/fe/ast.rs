use std::collections::HashMap;

use super::Sp;

pub struct TypeRegistry<'tr> {
    pub data: HashMap<String, Type<'tr>>,
}

const PRIMITIVES: [&'static str; 14] = [
    "string",
    "bool",
    "u8",
    "u16",
    "u32",
    "u64",
    "usize",
    "i8",
    "i16",
    "i32",
    "i64",
    "isize",
    "f32",
    "f64"
];

impl TypeRegistry<'_> {
    pub fn new() -> Self {
        let mut s = Self { 
            data: HashMap::new()
        };
        s.add_primitives();
        s
    }

    fn add_primitives(&mut self) {
        for p in &PRIMITIVES {
            self.data.insert(p.to_string(), Type {
                name: p,
                kind: TypeKind::Primitive
            });
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeKind<'tr> {
    Primitive,
    Unit,
    Struct { fields: HashMap<String, &'tr Type<'tr>> },
    TupleStruct { fields: Vec<&'tr Type<'tr>> },
    Function {
        params: Vec<&'tr Type<'tr>>,
        out: &'tr Type<'tr>
    }
}

#[derive(Debug, PartialEq)]
pub struct Type<'tr> {
    name: &'tr str,
    kind: TypeKind<'tr>,
}

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
pub enum AstNodeKind<'a, 'tr> {
    Value(ConstantValue<'a>),
    BinOp {
        a: AstNode<'a, 'tr>,
        b: AstNode<'a, 'tr>,
        op: Sp<'a, BinOp>
    },
    UnOp {
        op: Sp<'a, UnOp>,
        target: AstNode<'a, 'tr>
    },
    Error
}

impl<'a, 'tr> AstNodeKind<'a, 'tr> {
    pub fn get_start_end(&self) -> Option<(Sp<'a, ()>, Sp<'a, ()>)> {
        Some(match self {
            Self::BinOp { a, b, .. } => (a.ditch(), b.ditch()),
            Self::UnOp { op, target } => (op.ditch(), target.ditch()),
            _ => return None
        })
    }
}

pub type AstNode<'a, 'tr> = Sp<'a, RawAstNode<'a, 'tr>>;

#[derive(Debug)]
pub struct RawAstNode<'a, 'tr> {
    pub kind: Box<AstNodeKind<'a, 'tr>>,
    pub type_data: Option<&'tr Type<'tr>>,
}

impl<'a, 'tr> RawAstNode<'a, 'tr> {
    pub fn new(kind: AstNodeKind<'a, 'tr>) -> Self {
        Self {
            kind: Box::new(kind),
            type_data: None
        } 
    }
}

#[derive(Debug)]
pub enum ImportElement<'a> {
    Item(&'a str),
    Access(Vec<ImportElement<'a>>),
}

#[derive(Debug)]
pub enum MaybeTyped<'a, 'tr> {
    NotTyped,
    TypeProvided(&'a str),
    TypeResolved(&'tr Type<'tr>)
}

impl<'a, 'tr> MaybeTyped<'a, 'tr> {
    pub fn unwrap_type(self) -> &'tr Type<'tr> {
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
pub enum Statement<'a, 'tr> {
    Declare {
        id: &'a str,
        with_type: MaybeTyped<'a, 'tr>,
        value: AstNode<'a, 'tr>
    },
    Expression(AstNode<'a, 'tr>),
    Import {
        paths: ImportElement<'a>
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

pub fn print_tree(depth: u16, label: &str, node: &AstNode) {
    let s = format!("{}{}: ", beegstr(depth), label);
    match &*node.kind {
        AstNodeKind::BinOp { a, b, op } => {
            println!("{}BinOp({:?})", s, **op);
            print_tree(depth + 1, "a", &a);
            print_tree(depth + 1, "b", &b);
        },
        AstNodeKind::Value(v) => { println!("{}Value({:?})", s, v) },
        AstNodeKind::UnOp { op, target } => {
            println!("{}UnOp({:?})", s, **op);
            print_tree(depth + 1, "t", &target);
        }
        _ => println!("unknown")
    }
}
