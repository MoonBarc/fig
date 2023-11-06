use super::{lexer::LexError, Sp};

#[derive(Debug)]
pub enum CommentType {
    Regular,
    Doc,
    ModuleDoc
}

#[derive(Debug)]
pub enum Token<'a> {
    Identifier(&'a str),

    Comment(CommentType, &'a str), 

    // Literals
    String(&'a str),
    CompInt(i64),
    CompFloat(f64),
    
    // Operators and their respective = variants
    Add, AddEq, // +
    Sub, SubEq, // -
    Mul, MulEq, // *
    Div, DivEq, // /
    Pow, PowEq, // **
    Mod, ModEq, // %
    Dot, // .
    Try, // ?
    Not, // !
    Assign, // =

    // Comparisons
    Eq, NotEq, // == / !=
    Lt, LtEq, // <
    Gt, GtEq, // >

    // Logical
    // TODO: consider logical operators that don't short circuit
    // also consider bitwise operators
    And, // &&
    Or, // ||
    
    // Separators
    LParen, RParen, // ()
    LBrace, RBrace, // {}
    LBracket, RBracket, // []
    Comma, // ,
    Semicolon, // ;
    Newline, // Parser::advance deals with this for automatic semicolon insertion

    // Other
    At, // @
    Arrow, // ->

    // Keywords
    Let,
    Mut,
    If,
    Else,
    Fn,
    For,
    While,
    Loop,
    Break,
    Continue,
    Return,
    Enum,
    Struct,
    Import,
    Pub,
    Match,
    Default,

    // Keyword Values
    Nil,
    True,
    False,

    // Internal
    Error(LexError),
    Nothing
}

impl Token<'_> {
    pub(crate) fn nothing_span() -> Sp<'static, Self> {
        Sp {
            line: 0,
            col: 0,
            span: "nothing!",
            data: Token::Nothing 
        }
    }

    // TODO: Stuff for optional semicolons

    pub fn is_value(&self) -> bool {
        use Token::*;
        match self {
            CompInt(..) | CompFloat(..) | String(..) | Nil | True | False => true,
            _ => false
        }
    }

    pub fn can_end_stmt(&self) -> bool {
        use Token::*;
        match self {
            Break | Continue | Return => true,
            _ => self.is_value()
        }
    }

    pub fn can_start_stmt(&self) -> bool {
        todo!()
    }

    pub fn semicolon_inbetween(&self, next: &Self) -> bool {
        self.can_end_stmt() && next.can_start_stmt()
    }
}
