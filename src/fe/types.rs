use crate::fe::ast::UnOp;
use crate::fe::symbols::TypeProps;
use super::{ast::{ConstantValue, AstNodeKind, AstNode, CompFloat, CompInt, BinOp, Statement}, CompileError, Sp, symbols::{SymbolTable, PrimitiveType}};

pub fn type_check<'a>(symbols: &SymbolTable<'a>, ast: &mut AstNode<'a>) -> Vec<CompileError<'a>> {
    let mut errors = vec![];

    ast.type_data = Some(match &mut *ast.kind {
        AstNodeKind::Value(v) => match v {
            // hideous code incoming!
            ConstantValue::String(..) => symbols.get_primitive(PrimitiveType::String),
            ConstantValue::CompInt(i) => match i {
                CompInt::I8(..) => symbols.get_primitive(PrimitiveType::I8),
                CompInt::I16(..) => symbols.get_primitive(PrimitiveType::I16),
                CompInt::I32(..) => symbols.get_primitive(PrimitiveType::I32),
                CompInt::I64(..) => symbols.get_primitive(PrimitiveType::I64),
                CompInt::ISize(..) => symbols.get_primitive(PrimitiveType::ISize),
                CompInt::U8(..) => symbols.get_primitive(PrimitiveType::U8),
                CompInt::U16(..) => symbols.get_primitive(PrimitiveType::U16),
                CompInt::U32(..) => symbols.get_primitive(PrimitiveType::U32),
                CompInt::U64(..) => symbols.get_primitive(PrimitiveType::U64),
                CompInt::USize(..) => symbols.get_primitive(PrimitiveType::USize),
            },
            ConstantValue::CompFloat(f) => match f {
                CompFloat::F32(..) => symbols.get_primitive(PrimitiveType::F32),
                CompFloat::F64(..) => symbols.get_primitive(PrimitiveType::F64)
            },
            ConstantValue::Bool(..) => symbols.get_primitive(PrimitiveType::Bool),
            ConstantValue::Nil => todo!("sum types and lang items"),
        },
        AstNodeKind::BinOp { a, b, op } => {
            errors.append(&mut type_check(symbols, a));
            errors.append(&mut type_check(symbols, b));
            match **op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    // HACK: this assumes that everything is a number
                    if a.type_data != b.type_data {
                        panic!("type mismatch!"); // TODO: error handling
                    };
                    a.type_data.unwrap()
                },
                _ => todo!("binop semantic analysis is not fully implemented")
            }
        },
        AstNodeKind::UnOp { op, target } => {
            errors.append(&mut type_check(symbols, target));
            match op.data {
                UnOp::Negate => {
                    // only signed ints and floats can be negated
                    let t = target.type_data.unwrap();

                    let prim = symbols.get_primitive_type_from_id(t).get_props();
                    match prim {
                        TypeProps::Integer { signed, .. } => {
                            if !signed {
                                panic!("cannot negate an unsigned integer")
                            }
                        }
                        TypeProps::Float { .. } => { /* ok! */ }
                        TypeProps::Standalone => panic!("cannot negate a non-number")
                    }
                    t
                }
                UnOp::Not => {
                    let b_type = symbols.get_primitive(PrimitiveType::Bool);
                    if target.type_data != Some(b_type) {
                        panic!("can only apply `!` to boolean"); // TODO: error handling
                    }
                    b_type
                }
                _ => todo!("unop semantic analysis is not fully implemented")
            }
        },
        AstNodeKind::Error => todo!("fix your error for now"),
    });

    errors
}

pub fn type_check_block<'a>(symbols: &SymbolTable<'a>, block: &mut Vec<Statement<'a>>) -> Vec<CompileError<'a>> {
    let errs = vec![];
    for stmt in block {
        match stmt {
            Statement::Declare { id, with_type, value } => {
                type_check(&symbols, value);
                // TODO: error handling
                assert_eq!(with_type.unwrap_type(), value.type_data.unwrap(), "type mismatch!")
            },
            Statement::Expression(e) => {
                type_check(&symbols, e);
            },
            Statement::Return(e) => {
                type_check(&symbols, e);
            },
            Statement::Import { .. } | Statement::Error => { },
        };
    }
    errs
}
