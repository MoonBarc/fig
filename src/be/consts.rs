use crate::fe::ast::ConstantValue;

#[derive(Debug)]
pub struct ConstTable<'a> {
    pub consts: Vec<ConstantValue<'a>>
}

impl<'a> ConstTable<'a> {
    pub fn new() -> Self {
        Self {
            consts: vec![]
        }
    }

    pub fn add(&mut self, v: ConstantValue<'a>) -> usize {
        self.consts.push(v);
        self.consts.len() - 1
    }
}
