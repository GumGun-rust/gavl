use core::{
    ops::{
        Index,
        IndexMut,
    },
};


#[derive(Debug, Clone, Copy)]
pub enum Side{
    Left,
    Right
}

#[derive(Debug, Default)]
pub struct BinarySon<U>{
    content:[U; 2]
}

impl<U> Index<Side> for BinarySon<U>{
    type Output = U;
    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self.content[0],
            Side::Right => &self.content[1],
        }
    }
}

impl<U> IndexMut<Side> for BinarySon<U>{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Left => &mut self.content[0],
            Side::Right => &mut self.content[1],
        }
    }
}

#[cfg(feature = "into_precompiled")]
pub use into_precompiled::*;

#[cfg(feature = "into_precompiled")]
mod into_precompiled {
    
}

