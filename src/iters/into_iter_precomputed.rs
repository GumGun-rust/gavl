use std::{
    iter::Iterator,
    marker::PhantomData,

};

use super::{
    super::{
        structs::{
            Side,
        },
        Map,
        MapLink,
    },
};

use crate::{
    into_precomputed::PrecomputedIterNode,
};

/// # Dependant on feature into_precomputed
/// Return all the nodes in order of the keys, 
/// it has the index of the nodes that should be its sons assuming it is directly added to a linear
/// style data structure (`Vec` for example)
/// 
/// * This structure is generated by the method [`Map::into_iter_precomputed`][into_iter_precomputed]
/// 
/// [into_iter_precomputed]: Map::into_iter_precomputed
pub struct IntoIterPrecomp<KeyType:Ord, ContentType> {
    map: Map<KeyType, ContentType>,
    iter_data: IntoIterPrecompEnum<KeyType, ContentType>
}

pub(crate) enum IntoIterPrecompEnum<KeyType:Ord, ContentType> {
    NewIter,
    Iter{
        head_found: bool,
        next: Option<MapLink<KeyType, ContentType>>,
        phantom0: PhantomData<KeyType>,
        phantom1: PhantomData<ContentType>,
    }
}


impl<KeyType:Ord, ContentType> IntoIterPrecomp<KeyType, ContentType> {
    pub(crate) fn new(mut map:Map<KeyType, ContentType>) -> Self {
        map.calculate_indexes();
        IntoIterPrecomp{
            map: map,
            iter_data: IntoIterPrecompEnum::NewIter,
        }
        
    }
}

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    
    
    fn calculate_indexes(&mut self) {
        if self.size == 0 {
            return;
        }
        
        let mut pivot = self.head.unwrap();
        loop {
            let pivot_ref = unsafe{pivot.as_ref()};
            match pivot_ref.son[Side::Left] {
                Some(son) => {
                    pivot = son;
                },
                None => {
                    break;
                }
            }
        }
        
        let first_node = pivot;
        let pivot_mut = unsafe{pivot.as_mut()};
        pivot_mut.metadata.index = 0;
        //println!("{:#?} {}", pivot_mut.key, 0);
        let mut iter = 1;
        
        
        loop {
            match Self::next_node(pivot) {
                Some(mut node) => {
                    let node_mut = unsafe {node.as_mut()};
                    node_mut.metadata.index = iter;
                    pivot = node;
                    //println!("{:#?} {}", node_mut.key, iter);
                },
                None => {
                    break;
                }
            }
            iter += 1;
        }
        
        let mut pivot = first_node;
        
        //println!("\n\n\n\n\n\n\n");
        loop {
            let pivot_mut = unsafe{pivot.as_mut()};
            //println!("HOLA {:#?} {:#?} {}", pivot_mut.key, pivot_mut.content, pivot_mut.metadata.index);
            for side in [Side::Right, Side::Left] {
                match pivot_mut.son[side] {
                    Some(son) => {
                        let son_ref = unsafe{son.as_ref()};
                        //println!("si tiene hijo {:?}", 1);
                        pivot_mut.metadata.son_index[side] = Some(son_ref.metadata.index);
                    }
                    None => {}
                }
            
            }
            
            pivot = match Self::next_node(pivot) {
                Some(next) => next,
                None => {break;}
            }
        }
    }
    
}

impl<KeyType:Ord, ContentType> IntoIterPrecompEnum<KeyType, ContentType> {
    
}

impl<KeyType:Ord, ContentType> Iterator for IntoIterPrecomp<KeyType, ContentType> {
    type Item = PrecomputedIterNode<KeyType, ContentType>;
    
    fn next(&mut self) -> Option<Self::Item> {
        
        match self.iter_data {
            IntoIterPrecompEnum::NewIter => {
                let mut pivot = match self.map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                self.map.head = None;
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
                let holder = pivot;
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                let next = match holder_box.son[Side::Right] {
                    Some(mut next) => {
                        let next_mut = unsafe{next.as_mut()};
                        if let Some(mut father) = holder_box.father {
                            let father_mut = unsafe{father.as_mut()};
                            father_mut.son[Side::Left] = Some(next);
                            next_mut.father = Some(father);
                        } else {
                            next_mut.father = None;
                        }
                        Some(next)
                    },
                    None => {
                        if let Some(mut father) = holder_box.father {
                            let father_mut = unsafe{father.as_mut()};
                            father_mut.son[Side::Left] = None;
                        }
                        holder_box.father
                    },
                };
                self.iter_data = IntoIterPrecompEnum::Iter{next:next, head_found:false, phantom0:PhantomData, phantom1:PhantomData};
                
                Some(
                    PrecomputedIterNode{
                        head:false, 
                        key:holder_box.key, 
                        content:holder_box.content, 
                        prev_index:holder_box.metadata.son_index[Side::Left], 
                        next_index:holder_box.metadata.son_index[Side::Right]
                    }
                )
            },
            IntoIterPrecompEnum::Iter{
                ref mut next,
                ref mut head_found,
                ..
            } => {
                match next {
                    Some(holder) => {
                        let mut is_head = false;
                        let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                        *next = match holder_box.son[Side::Right] {
                            Some(mut pivot) => {
                                let pivot_mut = unsafe{pivot.as_mut()};
                                if let Some(mut father) = holder_box.father {
                                    let father_mut = unsafe{father.as_mut()};
                                    father_mut.son[Side::Left] = Some(pivot);
                                    pivot_mut.father = Some(father);
                                } else {
                                    if !*head_found {
                                        *head_found = true;
                                        is_head = true;
                                    }
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
                                //panic!();
                            },
                            None => {
                                if let Some(mut father) = holder_box.father {
                                    let father_mut = unsafe{father.as_mut()};
                                    father_mut.son[Side::Left] = None;
                                } else if !*head_found {
                                    *head_found = true;
                                    is_head = true;
                                }
                                //panic!();
                                holder_box.father
                            },
                        };
                        Some(
                            PrecomputedIterNode{
                                head:is_head, 
                                key:holder_box.key, 
                                content:holder_box.content, 
                                prev_index:holder_box.metadata.son_index[Side::Left], 
                                next_index:holder_box.metadata.son_index[Side::Right]
                            }
                        )
                    },
                    None => {
                        None
                    },
                } 
                
            }
            
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    
    #[test]
    fn test() {
        //posible that this crashes due to a repeted number in the rand
        let mut avl = Map::<u64,u64>::new();
        for content in 4+0..4+7+5+1000 {
            avl.add(content, 0).unwrap();
        }
        println!("{:#?}", &avl);
        let iter_level = avl.into_iter_precomputed().enumerate();
        print_type_of(&iter_level);
        for elem in iter_level {
            println!("{:?}", &elem);
            let (index, elem) = elem;
            println!("{:?} {:?} {:?} {:?} {:?} {:?}", index, elem.key, elem.content, elem.head, elem.prev_index, elem.next_index);
        }
        //panic!();
    }
}

