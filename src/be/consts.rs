use crate::fe::ast::ConstantValue;

pub struct ConstsTable<'a> {
    next_id: usize,
    pub consts: Vec<ConstantValue<'a>>
}

impl<'a> ConstsTable<'a> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            consts: vec![]
        }
    }

    pub fn add(&mut self, v: ConstantValue<'a>) -> usize {
        self.consts.push(v);
        self.consts.len() - 1
    }
}
