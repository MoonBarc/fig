use std::collections::HashMap;

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
    Bool(bool)
}

#[derive(Debug)]
pub enum AstNodeKind<'a> {
    Value(ConstantValue<'a>),
    If
}

#[derive(Debug)]
pub struct AstNode<'a, 'tr> {
    pub kind: AstNodeKind<'a>,
    pub type_data: Option<&'tr Type<'tr>>,

}

#[derive(Debug)]
pub enum ImportElement<'a> {
    Item(&'a str),
    Access(Vec<ImportElement<'a>>),
}

#[derive(Debug)]
pub enum Statement<'a, 'tr> {
    Declare {
        id: &'a str,
        with_type: Option<&'a str>,
        value: AstNode<'a, 'tr>
    },
    Expression(AstNode<'a, 'tr>),
    Import {
        paths: ImportElement<'a>
    }
}
