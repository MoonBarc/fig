//! The Backend

use self::consts::ConstTable;

pub mod ir;
pub mod irgen;
pub mod consts;
pub mod platform;
pub mod block;

#[derive(Debug)]
pub enum Item {
    Function {
        // TODO: function params here
        params: Vec<()>,
    }
}

pub struct CompUnit<'a>{
    pub prog: &'a str,
    pub consts: ConstTable<'a>,
    pub items: Vec<Item>
}
