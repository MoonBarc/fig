//! The Backend

use crate::fe::item::Item;

use self::consts::ConstTable;

pub mod ir;
pub mod irgen;
pub mod consts;
pub mod platform;
pub mod ralloc;

pub struct CompUnit<'a>{
    pub prog: &'a str,
    pub consts: ConstTable<'a>,
    pub items: Vec<Item<'a>>
}
