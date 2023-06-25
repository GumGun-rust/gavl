use std::{
    //iter::Iterator,
    marker::PhantomData,
};

use super::{
    IterRef,
    IterRefEnum,
    //LevelIter,
    super::{
        structs::{
            Side,
        },
        Map,
    },
};

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    pub fn iter_ref(&mut self) -> IterRef<KeyType, ContentType> {
        IterRef(
            IterRefEnum::NewIter(self)
        )
    }
}

impl<'a, KeyType:Ord, ContentType> IterRef<'a, KeyType, ContentType> {
    
}

impl<'a, KeyType:Ord, ContentType> Iterator for IterRef<'a, KeyType, ContentType> {
    type Item = (&'a KeyType, &'a ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterRefEnum::NewIter(map) => {
                let mut pivot = match map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                loop {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    match pivot_ref.son[Side::Left] {
                        None => {
                            *self = IterRef(IterRefEnum::Iter{current:pivot, phantom0:PhantomData, phantom1:PhantomData});
                            
                            return Some((&pivot_ref.key, &pivot_ref.content));
                        },
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        }
                    }
                }
            },
            IterRefEnum::Iter{
                ref mut current,
                ..
            } => {
                let pivot_or_nothing = Map::next_node(*current);
                match pivot_or_nothing {
                    None => None,
                    Some(pivot) => {
                        let pivot_ref = unsafe{pivot.as_ref()};
                        *current = pivot;
                        Some((&pivot_ref.key, &pivot_ref.content))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    
    #[test]
    fn test() {
        let mut avl = Map::<u64,u64>::new();
        for number in 4+0..4+7 {
            avl.add(number, 0).unwrap();
        }
        println!("{:#?}", &avl);
        let iter_level = avl.iter_ref_mut();//.enumerate();
        for elem in iter_level {
            print_type_of(&elem);
            println!("{:?}", &elem);
            *elem.1 += 1;
            println!("{:?}", &elem);
        }
        println!("{:#?}", &avl);
    }
}

