use std::{
    marker::PhantomData,
};

use super::{
    super::{
        structs::{
            Side,
        },
        MapLink,
        Map,
    },
};

#[cfg(feature = "unchecked_mut")]
pub struct IterRefMutUnchecked<'a, KeyType:Ord, ContentType> (
    IterRefMutUncheckedEnum<'a, KeyType, ContentType>
);

enum IterRefMutUncheckedEnum<'a, KeyType:Ord, ContentType> {
    NewIter(&'a mut Map<KeyType, ContentType>),
    Iter{
        current: MapLink<KeyType, ContentType>,
        phantom0: PhantomData<&'a mut KeyType>,
        phantom1: PhantomData<&'a mut ContentType>,
    }
}

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    pub fn iter_ref_mut_unchecked(&mut self) -> IterRefMutUnchecked<KeyType, ContentType> {
        IterRefMutUnchecked(
            IterRefMutUncheckedEnum::NewIter(self)
        )
    }
}

impl<'a, KeyType:Ord, ContentType> IterRefMutUnchecked<'a, KeyType, ContentType> {
    
}

impl<'a, KeyType:Ord, ContentType> Iterator for IterRefMutUnchecked<'a, KeyType, ContentType> {
    type Item = (&'a mut KeyType, &'a mut ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterRefMutUncheckedEnum::NewIter(map) => {
                let mut pivot = match map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                loop {
                    let pivot_mut = unsafe{pivot.as_mut()};
                    match pivot_mut.son[Side::Left] {
                        None => {
                            *self = IterRefMutUnchecked(IterRefMutUncheckedEnum::Iter{current:pivot, phantom0:PhantomData, phantom1:PhantomData});
                            
                            return Some((&mut pivot_mut.key, &mut pivot_mut.content));
                        },
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        }
                    }
                }
            },
            IterRefMutUncheckedEnum::Iter{
                ref mut current,
                ..
            } => {
                let pivot_or_nothing = Map::next_node(*current);
                match pivot_or_nothing {
                    None => None,
                    Some(mut pivot) => {
                        let pivot_mut = unsafe{pivot.as_mut()};
                        *current = pivot;
                        Some((&mut pivot_mut.key, &mut pivot_mut.content))
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
        let iter_level = avl.iter_ref_mut_unchecked();//.enumerate();
        for elem in iter_level {
            print_type_of(&elem);
            println!("{:?}", &elem);
            *elem.0 += 8;
            *elem.1 += 1;
            println!("{:?}", &elem);
        }
        println!("{:#?}", &avl);
    }
}

