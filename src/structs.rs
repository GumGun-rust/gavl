use std::{
    ops::{
        Index,
        IndexMut,
    },
    cmp::Ordering,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

#[derive(Debug, Default)]
pub struct BinarySon<U> {
    content:[U; 2]
}

impl TryFrom<Ordering> for Side {
    type Error = ();
    
    fn try_from(value: Ordering) -> Result<Self, Self::Error> {
        match value {
            Ordering::Less => Ok(Side::Left),
            Ordering::Greater => Ok(Side::Right),
            Ordering::Equal => Err(()),
        }
    }
}

impl Side {
    pub(crate) fn complement(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<U> Index<Side> for BinarySon<U> {
    type Output = U;
    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self.content[0],
            Side::Right => &self.content[1],
        }
    }
}

impl<U> IndexMut<Side> for BinarySon<U> {
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


#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn struct_conversions() {
        assert_eq!(Side::try_from(Ordering::Less).unwrap(), Side::Left);
        assert_eq!(Side::try_from(Ordering::Greater).unwrap(), Side::Right);
        println!("hola");
    }
}
