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
    
    // Other
    At, // @

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
}
