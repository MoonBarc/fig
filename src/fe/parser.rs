use std::mem;

use super::{token::Token, Sp, lexer::Lexer, ast::{self, AstNodeKind, RawAstNode, UnOp, Statement, ImportElement, AstNode, MaybeTyped, Reference}, CompileError};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Sp<'a, Token<'a>>,
    next: Sp<'a, Token<'a>>,
    panicking: bool, // TODO: error recovery
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

    pub fn parse(mut self) -> (Vec<Statement<'a>>, Vec<CompileError<'a>>) {
        let mut stmts = vec![];
        while !self.pick(&Token::Nothing) {
            let stmt = self.statement();
            stmts.push(stmt);
        }
        (stmts, self.errors)
    }

    fn statement(&mut self) -> Statement<'a> {
        let stmt = if self.pick(&Token::Import) {
            self.import()
        } else if self.pick(&Token::Let) {
            self.decl()
        } else if self.pick(&Token::Return) {
            self.ret()
        } else {
            Statement::Expression(self.top_parse())
        };
        if !self.pick(&Token::Semicolon) {
            self.error("expected `;` to end statement");
            return Statement::Error
        }
        stmt
    }
    
    pub fn parse_expr(mut self) -> (AstNode<'a>, Vec<CompileError<'a>>) {
        let thing = self.top_parse();
        (thing, self.errors)
    }

    fn unwrap_current_id_unchecked(&self) -> &'a str {
        match *self.current {
            Token::Identifier(i) => i,
            _ => unreachable!()
        }
    }

    fn ret(&mut self) -> Statement<'a> {
        let expr = self.top_parse();
        Statement::Return(expr)
    }

    fn decl(&mut self) -> Statement<'a> {
        if !self.pick(&Token::Identifier("")) {
            self.error("expected identifier to follow start of declaration");
            return Statement::Error
        }
        let id = self.unwrap_current_id_unchecked();
        let mut type_spec = MaybeTyped::NotTyped;
        if self.pick(&Token::Colon) {
            // yay, types!
            if !self.pick(&Token::Identifier("")) {
                // we can still try to continue without explicit type info, so we're not returning
                self.error("expected type identifier following start of type specification (`:`)");
            } else {
                type_spec = MaybeTyped::TypeProvided(self.unwrap_current_id_unchecked());
            }
        }
        if !self.pick(&Token::Assign) {
            self.error("expected `=` to follow name of declaration");
            return Statement::Error
        }
        let initializer = self.top_parse();
        Statement::Declare {
            id: Reference::Unresolved(id),
            with_type: type_spec,
            value: initializer 
        }
    }

    fn import(&mut self) -> Statement<'a> {
        let root = self.import_elem();
        Statement::Import {
            paths: root
        }
    }

    fn import_elem(&mut self) -> Vec<ImportElement<'a>> {
        match self.advance() {
            Token::Identifier(id) => {
                let id = *id;
                if self.pick(&Token::Dot) {
                    vec![ImportElement::Access(id, self.import_elem())]
                } else {
                    vec![ImportElement::Item(id)]
                }
            },
            Token::LParen => {
                let mut elems = vec![];
                loop {
                    if matches!(*self.next, Token::RParen) { break }
                    let mut elem = self.import_elem();
                    elems.append(&mut elem);
                    if !self.pick(&Token::Comma) { break }
                }
                if !self.pick(&Token::RParen) {
                    self.error("expected a closing `)` for import statement group");
                    return vec![]
                }
                elems
            },
            _ => {
                self.error("expected an identifier or `(` for import statement");
                vec![]
            }
        }
    }

    fn top_parse(&mut self) -> AstNode<'a> {
        self.parse_with_prec(prec::ASSIGN)
    }

    fn parse_with_prec(&mut self, prec: u8) -> AstNode<'a> {
        use Token::*;
        let mut node = match self.advance() {
            n if n.is_value() => self.value(),
            Identifier(..) => self.ident(),
            LParen => self.group(),
            Sub | Not => self.unary(prec),
            // Identifier(_) => self.identifier(),
            _ => {
                dbg!(&self.current, &self.next);
                panic!();
                return self.error("expected expression")
            }
        };

        while {
            let nprec = self.next.get_precedence();
            prec <= nprec
        } {
            node = match self.advance() {
                Add | AddEq | Sub | SubEq | Mul | MulEq | Div | DivEq
                    | Pow | PowEq | Mod | ModEq | Assign => self.binary(node, self.current.get_precedence() + 1),
                x => panic!("{:#?} has precedence but no associated infix operation", x)
            };
        }

        node 
    }

    fn ident(&mut self) -> AstNode<'a> {
        self.sp(AstNodeKind::Reference(
            Reference::Unresolved(self.unwrap_current_id_unchecked())
        ))
    }

    fn unary(&mut self, prec: u8) -> AstNode<'a> {
        let op = self.current.map(|d| match d {
            Token::Not => UnOp::Not,
            Token::Sub => UnOp::Negate,
            _ => todo!("thought it was a unop but it wasn't ({:#?})", d)
        });
        let target = self.parse_with_prec(prec);
        self.sp(AstNodeKind::UnOp {
            op ,
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
        let n = self.parse_with_prec(prec::ASSIGN);
        if !self.pick(&Token::RParen) {
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

        let rhs = self.parse_with_prec(prec);

        self.sp(AstNodeKind::BinOp { a: lhs, b: rhs, op })
    }
    
    // -- Utility Functions --

    fn sp(&mut self, node: AstNodeKind<'a>) -> AstNode<'a> {
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
        let mut next;
        loop {
            next = self.lexer.next().unwrap_or_else(|| Token::nothing_span());
            if let Token::Comment(..) = *next {
                continue
            }

            break
        }
        mem::swap(&mut self.next, &mut self.current);
        self.next = next;
        &self.current
    }

    fn pick(&mut self, tt: &Token) -> bool {
        if mem::discriminant(tt) == mem::discriminant(&self.next) {
            self.advance();
            true
        } else { false }
    }
}
