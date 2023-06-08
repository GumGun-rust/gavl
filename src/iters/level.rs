use std::{
    iter::Iterator,
    marker::PhantomData,
};
use super::{
    LevelIter,
    //LevelIterMut,
    super::{
        Map
    },
};


impl<T:Ord, U> Map<T, U> {
    pub fn level_iter(&mut self) -> LevelIter<T, U> {
        
        LevelIter{
            state:2,
            data_struct: self,
            current: self.head.unwrap(),
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
    
    /*
    pub fn level_iter_mut(&mut self) -> LevelIterMut<T, U> {
        LevelIterMut{
            state:2,
            data_struct: self,
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
    */
}

impl<'a, T:Ord, U> Iterator for LevelIter<'a, T, U> {
    type Item = (T, U);
    
    fn next(&mut self) -> Option<Self::Item> {
        
        if self.state != 0 {
            self.state -= 1;
            return Some(i32::from(self.state));
        }
        None
        //panic!();
        /*
        */
    }
}


