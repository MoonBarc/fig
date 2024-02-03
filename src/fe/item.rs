use super::ast::Statement;

pub enum Item<'a> {
    Function {
        code: Vec<Statement<'a>>,
        return_type: usize
    },
    Variable {
        ty: usize
    }
}
