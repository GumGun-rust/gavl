pub mod iter;
pub mod iter_mut;
pub mod into_iter;

#[cfg(any(feature = "unchecked_mut", doc))]
pub mod iter_ref_mut_unchecked;
#[cfg(any(feature = "unchecked_mut", doc))]
pub use iter_ref_mut_unchecked::*;

#[cfg(any(feature = "into_precomputed", doc))]
pub mod into_iter_precomputed;
#[cfg(any(feature = "into_precomputed", doc))]
pub use into_iter_precomputed::*;

use super::{
    structs::{
        Side,
    },
    Map,
    MapNode,
    MapLink,
};

use std::marker::PhantomData;


pub struct Iter<'a, KeyType:Ord, ContentType> (
    IterEnum<'a, KeyType, ContentType>
);



enum IterEnum<'a, KeyType:Ord, ContentType> {
    NewIter(&'a Map<KeyType, ContentType>),
    Iter{
        current: MapLink<KeyType, ContentType>,
        phantom0: PhantomData<&'a mut KeyType>,
        phantom1: PhantomData<&'a mut ContentType>,
    }
}



pub struct IterMut<'a, KeyType:Ord, ContentType> (
    IterMutEnum<'a, KeyType, ContentType>
);



enum IterMutEnum<'a, KeyType:Ord, ContentType> {
    NewIter(&'a mut Map<KeyType, ContentType>),
    Iter{
        current: MapLink<KeyType, ContentType>,
        phantom0: PhantomData<&'a mut KeyType>,
        phantom1: PhantomData<&'a mut ContentType>,
    }
}



pub struct IntoIter<KeyType:Ord, ContentType> {
    map: Map<KeyType, ContentType>,
    iter_data: IntoIterEnum<KeyType, ContentType>
}



pub(crate) struct EmptyIter<'a, KeyType:Ord, ContentType> {
    map: &'a mut Map<KeyType, ContentType>,
    iter_data: IntoIterEnum<KeyType, ContentType>
}



enum IntoIterEnum<KeyType:Ord, ContentType> {
    NewIter,
    Iter{
        next: Option<MapLink<KeyType, ContentType>>,
        phantom0: PhantomData<KeyType>,
        phantom1: PhantomData<ContentType>,
    },
    
}

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    
    fn next_node(current:MapLink<KeyType, ContentType>) -> Option<MapLink<KeyType, ContentType>> {
        let current_ref = unsafe{current.as_ref()};
        if let Some(mut pivot) = current_ref.son[Side::Right] {
            loop {
                let pivot_ref = unsafe{pivot.as_ref()};
                match pivot_ref.son[Side::Left] {
                    Some(son) => {
                        pivot = son;
                    },
                    None => {
                        return Some(pivot);
                    }
                }
            }
        }
        let mut pivot = current;
        loop {
            let side = MapNode::get_side(pivot);
            match side {
                Some(Side::Left) => {
                    return unsafe{pivot.as_ref()}.father;
                },
                Some(Side::Right) => {
                    pivot = unsafe{pivot.as_ref()}.father.expect("should have father");
                },
                None => {
                    return None;
                }
            }
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



