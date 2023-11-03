use crate::fe::lexer::Lexer;

mod fe;

fn main() {
    let prog = include_str!("../example.fig");
    let mut lexer = Lexer::new(&prog);
    
    let mut tok;

    loop {
        tok = lexer.next();
        let Some(tok) = tok else { break };
        println!("{}@{}:{} -> {:?}", tok.span, tok.line, tok.col, &*tok)
    }

    println!("o/");
}
