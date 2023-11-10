use std::collections::HashMap;

use super::{Sp, ast::{AstNode, AstNodeKind}};
 
macro_rules! primitives {
    ($($name:ident, $in_code:expr),*) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum PrimitiveType {
            $(
                $name,
            )*
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

primitives!(
    String, "string",
    Bool, "bool",
    U8, "u8",
    U16, "u16",
    U32, "u32",
    U64, "u64",
    USize, "usize",
    I8, "i8",
    I16, "i16",
    I32, "i32",
    I64, "i64",
    ISize, "isize",
    F32, "f32",
    F64, "f64"
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
