use std::{
    marker::PhantomData,
};

use super::{
    super::{
        structs::{
            Side,
        },
        Map,
    },
    IterMut,
    IterMutEnum,
};

impl<'a, KeyType:Ord, ContentType> IterMut<'a, KeyType, ContentType> {
    pub fn new(map:&'a mut Map<KeyType, ContentType>) -> Self {
        IterMut(
            IterMutEnum::NewIter(map)
        )
    }
}

impl<'a, KeyType:Ord, ContentType> IterMut<'a, KeyType, ContentType> {
    
}

impl<'a, KeyType:Ord, ContentType> Iterator for IterMut<'a, KeyType, ContentType> {
    type Item = (&'a KeyType, &'a mut ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterMutEnum::NewIter(map) => {
                let mut pivot = match map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                loop {
                    let pivot_mut = unsafe{pivot.as_mut()};
                    match pivot_mut.son[Side::Left] {
                        None => {
                            *self = IterMut(IterMutEnum::Iter{current:pivot, phantom0:PhantomData, phantom1:PhantomData});
                            
                            return Some((&pivot_mut.key, &mut pivot_mut.content));
                        },
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        }
                    }
                }
            },
            IterMutEnum::Iter{
                ref mut current,
                ..
            } => {
                let pivot_or_nothing = Map::next_node(*current);
                match pivot_or_nothing {
                    None => None,
                    Some(mut pivot) => {
                        let pivot_mut = unsafe{pivot.as_mut()};
                        *current = pivot;
                        Some((&pivot_mut.key, &mut pivot_mut.content))
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
        let iter_level = avl.iter_mut();//.enumerate();
        for elem in iter_level {
            print_type_of(&elem);
            println!("{:?}", &elem);
            *elem.1 += 1;
            println!("{:?}", &elem);
        }
        println!("{:#?}", &avl);
    }
}

