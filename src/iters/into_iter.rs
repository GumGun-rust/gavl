use std::{
    marker::PhantomData,
};

use super::{
    super::{
        structs::{
            Side,
        },
        Map,
        MapLink,
        MapNode,
    },
    IntoIter,
    IntoIterEnum,
    EmptyIter,
};



impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    
    pub(crate) fn empty_iter(&mut self) -> EmptyIter<KeyType, ContentType> {
        EmptyIter{
            map: self,
            iter_data: IntoIterEnum::NewIter,
        }
    }
    
}

    

impl<KeyType:Ord, ContentType> IntoIter<KeyType, ContentType> {
    pub(crate) fn new(map:Map<KeyType, ContentType>) -> Self {
        IntoIter{
            map: map,
            iter_data: IntoIterEnum::NewIter,
        }
    }
}



impl<KeyType:Ord, ContentType> Iterator for IntoIter<KeyType, ContentType> {
    type Item = (KeyType, ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        
        match self.iter_data {
            IntoIterEnum::NewIter => {
                let holder = IntoIterEnum::get_first(&mut self.map)?;
                self.map.head = None;
                self.iter_data = IntoIterEnum::Iter{next:Some(holder), phantom0:PhantomData, phantom1:PhantomData};
                self.iter_data.next();
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                Some((holder_box.key, holder_box.content))
            },
            IntoIterEnum::Iter{
                ..
            } => {
                let holder = self.iter_data.next()?;
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                Some((holder_box.key, holder_box.content))
            },
        }
    }
}

impl<'a, KeyType:Ord, ContentType> Iterator for EmptyIter<'a, KeyType, ContentType> {
    type Item = ();
    
    fn next(&mut self) -> Option<Self::Item> {
        
        match self.iter_data {
            IntoIterEnum::NewIter => {
                let holder = IntoIterEnum::get_first(self.map)?;
                self.map.head = None;
                self.map.size = 0;
                self.iter_data = IntoIterEnum::Iter{next:Some(holder), phantom0:PhantomData, phantom1:PhantomData};
                self.iter_data.next();
                
                MapNode::free_node(holder);
                Some(())
            },
            IntoIterEnum::Iter{
                ..
            } => {
                let holder = self.iter_data.next()?;
                MapNode::free_node(holder);
                Some(())
            },
        }
    }
}

impl<KeyType:Ord, ContentType> IntoIterEnum<KeyType, ContentType> {
    fn get_first(map: &mut Map<KeyType, ContentType>) -> Option<MapLink<KeyType, ContentType>> {
        let mut pivot = map.head?; 
        loop{
            let pivot_ref = unsafe{pivot.as_ref()};
            match pivot_ref.son[Side::Left] {
                Some(new_pivot) => {
                    pivot = new_pivot;
                },
                None => {
                    break;
                }
            }
        }
        Some(pivot)
    }
}

impl<KeyType:Ord, ContentType> Iterator for IntoIterEnum<KeyType, ContentType> {
    type Item = MapLink<KeyType, ContentType>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let IntoIterEnum::Iter{ ref mut next, .. } = self {
            let holder = next.clone()?;
            
            let holder_ref = unsafe{holder.as_ref()};
            *next = match holder_ref.son[Side::Right] {
                Some(mut pivot) => {
                    let pivot_mut = unsafe{pivot.as_mut()};
                    if let Some(mut father) = holder_ref.father {
                        let father_mut = unsafe{father.as_mut()};
                        father_mut.son[Side::Left] = Some(pivot);
                        pivot_mut.father = Some(father);
                    } else {
                        pivot_mut.father = None;
                    }
                    
                    loop {
                        let pivot_mut = unsafe{pivot.as_mut()};
                        match pivot_mut.son[Side::Left] {
                            Some(next_pivot) => {
                                pivot = next_pivot;
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    Some(pivot)
                },
                None => {
                    if let Some(mut father) = holder_ref.father {
                        let father_mut = unsafe{father.as_mut()};
                        father_mut.son[Side::Left] = None;
                    }
                    holder_ref.father
                },
            };
            return Some(holder)
        }
        None
    }
    
}
    
    
#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    
    #[test]
    fn test() {
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 4+0..4+7+5 {
            avl.add(elem, 0).unwrap();
        }
        println!("{:#?}", &avl);
        
        let iter_level = avl.into_iter();
        print_type_of(&iter_level);
        for elem in iter_level {
            println!("{:?}", &elem);
        }
    }
    
    #[test]
    fn empty() {
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 4+0..4+7+5 {
            avl.add(elem, 0).unwrap();
        }
        println!("{:#?}", &avl);
        
        let empty_iter = avl.empty_iter();
        for elem in empty_iter {
            println!("{:?}", elem);
        }
        println!("{:#?}", &avl);
        //todo!();
    }
}

