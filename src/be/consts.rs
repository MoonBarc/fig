use crate::fe::ast::ConstantValue;

pub struct ConstTable<'a> {
    next_id: usize,
    pub consts: Vec<ConstantValue<'a>>
}

impl<'a> ConstTable<'a> {
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
