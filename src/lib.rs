mod structs;
mod balance;
mod traits;
mod iters;
mod errors;
use errors::AvlError;
pub use errors::*;

#[cfg(test)]
mod test;

#[cfg(feature = "into_precompiled")]
pub use into_precompiled::*;

#[cfg(feature = "into_precompiled")]
mod into_precompiled;

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
    metadata: into_precompiled::PrecompiledMetadata,
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
            #[cfg(feature = "into_precompiled")]
            metadata:into_precompiled::PrecompiledMetadata::default(),
        }))).expect("system ran out of memory");
        
        //println!("{:?}", new_node);
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                Ok(())
            },
            Some(data) => {
                if !MapNode::insert_node(data, new_node) {
                    return Err(AvlError::KeyOcupied);
                }
                self.size += 1;
                self.compute_height(new_node);
                Ok(())
            }
        }
        
    }
    
    pub fn empty(&mut self) {
        let empty_iter = self.empty_iter();
        for _elem in empty_iter {
            //just drop them
        }
    }
    
    pub fn get(&mut self, key:KeyType) -> Result<&ContentType, AvlError> {
        panic!();
    }
    
    pub fn get_mut(&mut self, key:KeyType) -> Result<&mut ContentType, AvlError> {
        panic!();
    }
    
    pub fn delete(&mut self, key:KeyType) -> Result<ContentType, AvlError> {
        panic!();
    }
    
    pub fn len(&self) -> usize {
        self.size
    }

}


