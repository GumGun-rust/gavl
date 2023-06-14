use std::{
    //iter::Iterator,
    marker::PhantomData,
};

use super::{
    OrderIterRef,
    //LevelIter,
    super::{
        structs::{
            Side,
        },
        Map,
    },
};

impl<T:Ord, U> Map<T, U> {
    pub fn order_iter_ref(&self) -> OrderIterRef<T, U> {
        OrderIterRef{
            started: false,
            current: self.head,
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
}

impl<'a, T:Ord, U> OrderIterRef<'a, T, U> {
    
}

impl<'a, T:Ord, U> Iterator for OrderIterRef<'a, T, U> {
    type Item = (&'a T, &'a U);
    
    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            match self.current {
                None => {
                    None
                },
                Some(mut pivot) => {
                    loop {
                        let pivot_ref = unsafe{pivot.as_ref()};
                        match pivot_ref.son[Side::Left] {
                            None => {
                                self.current = Some(pivot);
                                return Some((&pivot_ref.key, &pivot_ref.content));
                            },
                            Some(new_pivot) => {
                                pivot = new_pivot;
                            }
                        }
                    }
                }
            }
        } else {
            let pivot_or_nothing = Map::next_node(self.current.unwrap());
            match pivot_or_nothing {
                None => None,
                Some(pivot) => {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    self.current = Some(pivot);
                    Some((&pivot_ref.key, &pivot_ref.content))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut avl = Map::<String,u64>::new();
        for number in 4+0..4+15 {
            avl.add(number.to_string(), 0).unwrap();
        }
        let iter_level = avl.order_iter_ref();//.enumerate();
        //avl.add("test".to_owned(), 112);
        for elem in iter_level {
            println!("{:?}", elem);
        }
    }
    
    

}
