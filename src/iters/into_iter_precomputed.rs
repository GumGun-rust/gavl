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
    IntoIterEnum,
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
    head_found: bool,
    iter_data: IntoIterEnum<KeyType, ContentType>
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
        let mut iter = 1;
        
        loop {
            match Self::next_node(pivot) {
                Some(mut node) => {
                    let node_mut = unsafe {node.as_mut()};
                    node_mut.metadata.index = iter;
                    pivot = node;
                },
                None => {
                    break;
                }
            }
            iter += 1;
        }
        
        let mut pivot = first_node;
        
        loop {
            let pivot_mut = unsafe{pivot.as_mut()};
            for side in [Side::Right, Side::Left] {
                match pivot_mut.son[side] {
                    Some(son) => {
                        let son_ref = unsafe{son.as_ref()};
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




impl<KeyType:Ord, ContentType> IntoIterPrecomp<KeyType, ContentType> {
    pub(crate) fn new(mut map:Map<KeyType, ContentType>) -> Self {
        map.calculate_indexes();
        Self{
            map: map,
            head_found: false,
            iter_data: IntoIterEnum::NewIter,
        }
    }
}



impl<KeyType:Ord, ContentType> Iterator for IntoIterPrecomp<KeyType, ContentType> {
    type Item = PrecomputedIterNode<KeyType, ContentType>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_data {
            IntoIterEnum::NewIter => {
                let holder = IntoIterEnum::get_first(&mut self.map)?;
                self.map.head = None;
                self.iter_data = IntoIterEnum::Iter{next:Some(holder), phantom0:PhantomData, phantom1:PhantomData};
                self.iter_data.next();
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                let mut head = false;
                if let None = holder_box.father {
                    head = true;
                    self.head_found = true;
                }
                let holder_node = PrecomputedIterNode{
                    key: holder_box.key,
                    content: holder_box.content,
                    head: head,
                    prev_index: holder_box.metadata.son_index[Side::Left],
                    next_index: holder_box.metadata.son_index[Side::Right],
                };
                Some(holder_node)
            },
            IntoIterEnum::Iter{
                ..
            } => {
                let holder = self.iter_data.next()?;
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                let mut head = false;
                if let None = holder_box.father {
                    if !self.head_found {
                        head = true;
                        self.head_found = true;
                    }
                }
                let holder_node = PrecomputedIterNode{
                    key: holder_box.key,
                    content: holder_box.content,
                    head: head,
                    prev_index: holder_box.metadata.son_index[Side::Left],
                    next_index: holder_box.metadata.son_index[Side::Right],
                };
                Some(holder_node)
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
        //posible that this crashes due to a repeted number in the rand
        let mut avl = Map::<u64,u64>::new();
        for content in 4+0..4+7+5+100 {
            avl.add(content, 0).unwrap();
        }
        println!("{:#?}", &avl);
        let iter_level = avl.into_iter_precomputed().enumerate();
        print_type_of(&iter_level);
        for (index, elem) in iter_level {
            println!("{} {:?}", index, &elem);
            //println!("{:?} {:?} {:?} {:?} {:?} {:?}", index, elem.key, elem.content, elem.head, elem.prev_index, elem.next_index);
        }
        
    }
}

