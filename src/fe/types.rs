use super::{ast::{ConstantValue, AstNodeKind, AstNode, CompFloat, CompInt, BinOp}, CompileError, symbols::{SymbolTable, PrimitiveType}};

pub fn type_check<'a, 'sy>(symbols: &SymbolTable<'a>, ast: &mut AstNode<'a>) -> Vec<CompileError<'a>> {
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
                        todo!("error handling");
                    };
                    a.type_data.unwrap()
                },
                _ => todo!("binop semantic analysis is not fully implemented")
            }
        },
        AstNodeKind::UnOp { op, target } => {
            errors.append(&mut type_check(symbols, target));
            todo!("todo=unop")
        },
        AstNodeKind::Error => todo!(),
    });

    errors
}
