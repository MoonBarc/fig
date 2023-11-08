use std::mem;

use super::{token::Token, Sp, lexer::Lexer, ast::{self, AstNodeKind, RawAstNode, UnOp}, CompileError};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Sp<'a, Token<'a>>,
    next: Sp<'a, Token<'a>>,
    panicking: bool,
    errors: Vec<CompileError<'a>>,
}

macro_rules! precs {
    ($($prec:ident: $num:expr),+) => {
        pub mod prec {
            $(
            pub const $prec: u8 = $num;
            )*
        }
    };
}

// all AstNodes made in the parser do not reference the type registry, hence the 'static
pub type AstNode<'a> = ast::AstNode<'a, 'static>;

precs!(
    NONE: 0,
    ASSIGN: 1,
    OR: 2,
    AND: 3,
    EQ: 4,
    COMP: 5,
    TERM: 6,
    FACTOR: 7,
    POW: 8,
    UNARY: 9,
    CALL: 10,
    PRIMARY: 11
);

impl<'a> Parser<'a> {
    pub fn new(prog: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(prog);
        let first = lexer.next();
        Self {
            next: first.unwrap(),
            current: Token::nothing_span(),
            lexer,
            errors: Vec::new(),
            panicking: false
        } 
    }

    // pub fn parse(self) -> Vec<Statement> {
    //     
    // }
    
    pub fn parse_expr(mut self) -> (AstNode<'a>, Vec<CompileError<'a>>) {
        let thing = self.parse_with_prec(prec::PRIMARY);
        (thing, self.errors)
    }

    fn parse_with_prec(&mut self, prec: u8) -> AstNode<'a> {
        use Token::*;
        let mut node = match self.advance() {
            n if n.is_value() => self.value(),
            LParen => self.group(),
            Sub | Not => self.unary(prec),
            // Identifier(_) => self.identifier(),
            _ => return self.error("expected expression")
        };

        while {
            let nprec = self.next.get_precedence();
            nprec > prec::NONE && prec > nprec
        } {
            node = match self.advance() {
                Add | AddEq | Sub | SubEq | Mul | MulEq | Div | DivEq
                    | Pow | PowEq | Mod | ModEq => self.binary(node, prec),
                x => panic!("{:#?} has precedence but no associated infix operation", x)
            };
        }

        node 
    }

    fn unary(&mut self, prec: u8) -> AstNode<'a> {
        let op = self.current.map(|d| match d {
            Token::Not => UnOp::Not,
            Token::Sub => UnOp::Negate,
            _ => todo!("thought it was a unop but it wasn't ({:#?})", d)
        });
        let target = self.parse_with_prec(prec - 1);
        self.sp(AstNodeKind::UnOp {
            op,
            target 
        })
    }

    fn value(&mut self) -> AstNode<'a> {
        self.sp(AstNodeKind::Value(match *self.current {
            // TODO: don't assume that everything is an i64
            Token::CompInt(ci) => ast::ConstantValue::CompInt(ast::CompInt::I64(ci)),
            Token::CompFloat(cf) => ast::ConstantValue::CompFloat(ast::CompFloat::F64(cf)),
            Token::String(s) => ast::ConstantValue::String(s),
            Token::True => ast::ConstantValue::Bool(true),
            Token::False => ast::ConstantValue::Bool(false),
            Token::Nil => ast::ConstantValue::Nil,
            Token::Struct => todo!("structs not implemented"), // TODO: implement structs
            _ => panic!("thought it was a value but it wasn't")
        }))
    }
    
    fn group(&mut self) -> AstNode<'a> {
        let n = self.parse_with_prec(prec::PRIMARY);
        if !self.pick(Token::RParen) {
            return self.error("expected `)` to end group")
        };
        n
    }

    fn binary(&mut self, lhs: AstNode<'a>, prec: u8) -> AstNode<'a> {
        macro_rules! binop_equiv {
            ($d:expr, $($id:ident),*) => {
                match $d {
                    $(
                        Token::$id => ast::BinOp::$id,
                    )*
                    _ => panic!("thought it was a binop but it wasn't")
                }
            };
        }
        let op = self.current.map(|d| binop_equiv!(d,
            Assign,

            Add, AddEq,
            Sub, SubEq,
            Div, DivEq,
            Mul, MulEq,
            Pow, PowEq,
            Mod, ModEq,
            
            NotEq, Eq,
            Gt, GtEq,
            Lt, LtEq,

            And, Or
        ));

        let rhs = self.parse_with_prec(prec - 1);

        self.sp(AstNodeKind::BinOp { a: lhs, b: rhs, op })
    }
    
    // -- Utility Functions --

    fn sp(&mut self, node: AstNodeKind<'a, 'static>) -> AstNode<'a> {
        let data = RawAstNode::new(node);
        if let Some((start, end)) = data.kind.get_start_end() {
            return Sp {
                line: start.line,
                col: start.col,
                span: start.span.start..end.span.end,
                of: start.of,
                data
            }
        };
        Sp {
            line: self.current.line,
            col: self.current.col,
            span: self.current.span.clone(),
            of: self.lexer.prog,
            data
        }
    }

    fn error<T: ToString>(&mut self, err: T) -> AstNode<'a> {
        self.advance();
        let node = self.sp(AstNodeKind::Error);
        self.errors.push(CompileError {
            span: &node.of[node.span.clone()],
            message: err.to_string()
        });
        node
    }

    fn advance(&mut self) -> &Token<'a> {
        let next = self.lexer.next().unwrap_or_else(|| Token::nothing_span());
        mem::swap(&mut self.next, &mut self.current);
        self.next = next;
        &self.current
    }

    fn pick(&mut self, tt: Token) -> bool {
        if mem::discriminant(&tt) == mem::discriminant(&self.next) {
            self.advance();
            true
        } else { false }
    }
}
