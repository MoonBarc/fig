use std::collections::HashMap;

use super::{Sp, ast::{AstNode, AstNodeKind}};

pub enum TypeProps {
    Integer {
        signed: bool,
        /// A zero value represents a pointer-width number
        bits: u8
    },
    Float {
        bits: u8
    },
    Standalone
}

impl TypeProps {
    pub fn nothing() -> TypeProps { TypeProps::Standalone }
    pub fn number(s: bool, b: u8) -> TypeProps {
        TypeProps::Integer {
            signed: s,
            bits: b
        }
    }
    pub fn float(b: u8) -> TypeProps {
        TypeProps::Float {
            bits: b
        }
    }
}

macro_rules! primitives {
    ($($name:ident, $in_code:expr, $props:expr),*) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum PrimitiveType {
            $(
                $name,
            )*
        }

        impl PrimitiveType {
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(
                    PrimitiveType::$name => $in_code,
                    )*
                }
            }

            pub fn get_props(&self) -> TypeProps {
                match self {
                    $(
                    PrimitiveType::$name => $props,
                    )*
                }
            }
        }

        fn add_primitives(t: &mut SymbolTable) {
            $(
                let id = t.add(
                    Sp::builtin(Symbol::Type(
                        Type {
                            kind: TypeKind::Primitive,
                            name: $in_code
                        }   
                    ))
                );
                t.primitive_map.insert(PrimitiveType::$name, id);
            )*
        }
    };
}

//  Variant | In code | Properties
primitives!(
    String, "string",   TypeProps::nothing(),
    Bool,   "bool",     TypeProps::nothing(),
    U8,     "u8",       TypeProps::number(false, 8),
    U16,    "u16",      TypeProps::number(false, 16),
    U32,    "u32",      TypeProps::number(false, 32),
    U64,    "u64",      TypeProps::number(false, 64),
    USize,  "usize",    TypeProps::number(false, 0),
    I8,     "i8",       TypeProps::number(true, 8),
    I16,    "i16",      TypeProps::number(true, 16),
    I32,    "i32",      TypeProps::number(true, 32),
    I64,    "i64",      TypeProps::number(true, 64),
    ISize,  "isize",    TypeProps::number(true, 0),
    F32,    "f32",      TypeProps::float(32),
    F64,    "f64",      TypeProps::float(64)
);

#[derive(Debug, PartialEq)]
pub enum TypeKind<'a> {
    Primitive,
    Unit,
    Struct { fields: HashMap<&'a str, usize> },
    TupleStruct { fields: Vec<usize> },
    Function {
        params: Vec<usize>,
        out: usize 
    }
}

#[derive(Debug, PartialEq)]
pub struct Type<'a> {
    name: &'a str,
    kind: TypeKind<'a>,
}

#[derive(Debug)]
pub enum Symbol<'a> {
    Variable { ty: usize },
    Type(Type<'a>)
}

pub struct SymbolTable<'a> {
    next_id: usize,
    pub tbl: HashMap<usize, Sp<'a, Symbol<'a>>>,
    pub primitive_map: HashMap<PrimitiveType, usize>
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        let mut s = Self {
            next_id: 0,
            tbl: HashMap::new(),
            primitive_map: HashMap::new()
        };
        add_primitives(&mut s);
        s
    }

    fn allocate_id(&mut self) -> usize {
        let i = self.next_id;
        self.next_id += 1;
        i
    }

    pub fn get_primitive(&self, prim: PrimitiveType) -> usize {
        *self.primitive_map.get(&prim).unwrap()
    }

    pub fn get_primitive_type_from_id(&self, t: usize) -> &PrimitiveType {
        self.primitive_map.iter().find(|(_, a)| *a == &t).unwrap().0
    }

    pub fn add(&mut self, s: Sp<'a, Symbol<'a>>) -> usize {
        let id = self.allocate_id();
        self.tbl.insert(id, s);
        id
    }
}

#[derive(Debug)]
pub struct ScopeItem<'a> {
    name: &'a str,
    depth: usize
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub items: Vec<ScopeItem<'a>>
}

pub fn resolve(scope: &Scope, ast: &mut AstNode) {
    match &mut *ast.kind {
        AstNodeKind::Value(..) | AstNodeKind::Error => {}, // nothing to do
        AstNodeKind::BinOp { ref mut a, ref mut b, .. } => {
            resolve(scope, a);
            resolve(scope, b);
        },
        AstNodeKind::UnOp { target, .. } => todo!(),
    }
}
