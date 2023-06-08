use core::{
    ops::{
        Index,
        IndexMut,
    },
};

pub use super::{
    Node,
};

#[derive(Debug, Clone, Copy)]
pub enum Side{
    Left,
    Right
}

#[derive(Default)]
pub struct BinarySon<T>{
    content:[T; 2]
}

impl<T> Index<Side> for BinarySon<T>{
    type Output = T;
    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self.content[0],
            Side::Right => &self.content[1],
        }
    }
}

impl<T> IndexMut<Side> for BinarySon<T>{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Left => &mut self.content[0],
            Side::Right => &mut self.content[1],
        }
    }
}
