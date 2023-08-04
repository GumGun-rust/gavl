use std::{
    marker::PhantomData, };

use super::{
    super::{
        structs::{
            Side,
        },
        Map,
    },
    Iter,
    IterEnum,
    //LevelIter,
};

impl<'a, KeyType:Ord, ContentType> Iter<'a, KeyType, ContentType> {
    pub(crate) fn new(map:&'a Map<KeyType, ContentType>) -> Iter<'a, KeyType, ContentType> {
        Iter(
            IterEnum::NewIter(map)
        )
    }
}

impl<'a, KeyType:Ord, ContentType> Iter<'a, KeyType, ContentType> {
    
}

impl<'a, KeyType:Ord, ContentType> Iterator for Iter<'a, KeyType, ContentType> {
    type Item = (&'a KeyType, &'a ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterEnum::NewIter(map) => {
                let mut pivot = match map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                loop {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    match pivot_ref.son[Side::Left] {
                        None => {
                            *self = Iter(IterEnum::Iter{current:pivot, phantom0:PhantomData, phantom1:PhantomData});
                            
                            return Some((&pivot_ref.key, &pivot_ref.content));
                        },
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        }
                    }
                }
            },
            IterEnum::Iter{
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
        let iter_level = avl.iter();//.enumerate();
        for elem in iter_level {
            print_type_of(&elem);
            println!("{:?}", &elem);
            println!("{:?}", &elem);
        }
        println!("{:#?}", &avl);
    }
}

