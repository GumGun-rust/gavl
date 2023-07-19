mod structs;
mod balance;
mod traits;
mod iters;
mod errors;
mod into_precompiled;

use errors::AvlError;
pub use errors::*;

#[cfg(test)]
mod test;


use std::{
    ptr::NonNull,
};

pub struct Map<KeyType:Ord, ContentType>{
    head: Option<MapLink<KeyType, ContentType>>,
    size: usize,
}

struct MapNode<KeyType:Ord, ContentType>{
    key: KeyType,
    content: ContentType,
    father: Option<MapLink<KeyType,ContentType>>,
    depth: structs::BinarySon<i32>,
    son: structs::BinarySon<Option<MapLink<KeyType,ContentType>>>,
    #[cfg(feature = "into_precompiled")]
    metadata: into_precompiled::FeatureField,
}

type MapLink<KeyType, ContentType> = NonNull<MapNode<KeyType, ContentType>>;

/*
pub struct Set<KeyType:Ord>{
    head: Option<SetLink<KeyType>>,
    size: u64,
}

pub struct SetNode<KeyType:Ord>{
    content: KeyType,
    father: Option<SetLink<KeyType>>,
    depth: structs::BinarySon<i32>,
    son: structs::BinarySon<Option<SetLink<KeyType>>>,
    #[cfg(feature = "into_precompiled")]
    index: u64,
}

type SetLink<KeyType> = NonNull<SetNode<KeyType>>;
*/

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType>{
    
    

    pub fn new() -> Self {
        Self{head:None ,size:0}
    }
    
    

    pub fn add(&mut self, key:KeyType, content:ContentType) -> Result<(), AvlError> {
        let new_node = NonNull::new(Box::into_raw(Box::new(MapNode{
            key,
            content,
            father:None,
            depth: structs::BinarySon::default(),
            son: structs::BinarySon::default(),
            metadata:into_precompiled::FeatureField::default(),
        }))).expect("system ran out of memory");
        
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                Ok(())
            },
            Some(data) => {
                if let Err(_) = MapNode::insert_node(data, new_node) {
                    return Err(AvlError::KeyOcupied);
                }
                self.size += 1;
                self.compute_balance_additive(new_node);
                Ok(())
            }
        }
        
    }
    
    
    
    pub fn insert(&mut self) -> Option<ContentType> {
        panic!();
    }
    

    pub fn empty(&mut self) {
        let empty_iter = self.empty_iter();
        for _elem in empty_iter {
            //just drop them
        }
    }
    
    

    pub fn get(&mut self, key:&KeyType) -> Result<&ContentType, AvlError> {
        let pivot = match self.head {
            None => {return Err(AvlError::NotFound);}
            Some(data) => data,
        };
        let node = MapNode::find_node(key, pivot).ok_or(AvlError::NotFound)?;
        let node_ref = unsafe{node.as_ref()};
        Ok(&node_ref.content)
    }
    
    pub fn get_mut(&mut self, key:&KeyType) -> Result<&mut ContentType, AvlError> {
        let pivot = match self.head {
            None => {return Err(AvlError::NotFound);}
            Some(data) => data,
        };
        let mut node = MapNode::find_node(key, pivot).ok_or(AvlError::NotFound)?;
        let node_mut = unsafe{node.as_mut()};
        Ok(&mut node_mut.content)
    }



    pub fn remove(&mut self, key:&KeyType) -> Result<ContentType, AvlError> {
        match self.size {
            0 => {
                Err(AvlError::NotFound)
            },
            1 => {
                let head = self.head.unwrap();
                let head_ref = unsafe{head.as_ref()};
                if !head_ref.key.cmp(key).is_eq() {
                    return Err(AvlError::NotFound);
                }
                self.size = 0;
                self.head = None;
                let target = unsafe{Box::from_raw(head.as_ptr())};
                Ok(target.content)
            }
            _ => {
                let target = MapNode::find_node(key, self.head.unwrap()).ok_or(AvlError::NotFound)?;
                let balance_pivot = self.compute_subtraccion_pivot(target);
                self.compute_balance_subtractive(balance_pivot);
                self.size -= 1;
                let target = unsafe{Box::from_raw(target.as_ptr())};
                Ok(target.content)
            }
        }
    }
    
    

    pub fn delete(&mut self, key:&KeyType) -> Result<(), AvlError> {
        match self.size {
            0 => {
                Err(AvlError::NotFound)
            },
            1 => {
                let head = self.head.unwrap();
                let head_ref = unsafe{head.as_ref()};
                if !head_ref.key.cmp(key).is_eq() {
                    return Err(AvlError::NotFound);
                }
                self.size = 0;
                self.head = None;
                MapNode::free_node(head);
                Ok(())
            }
            _ => {
                let target = MapNode::find_node(key, self.head.unwrap()).ok_or(AvlError::NotFound)?;
                let balance_pivot = self.compute_subtraccion_pivot(target);
                self.compute_balance_subtractive(balance_pivot);
                self.size -= 1;
                MapNode::free_node(target);
                Ok(())
            }
        }
    }
    
    

    pub fn len(&self) -> usize {
        self.size
    }

    

}


