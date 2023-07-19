//! # Hola
//! Test long test

mod structs;
mod balance;
mod traits;
mod iters;

mod errors;
mod into_precompiled;


#[doc(inline)]
use errors::Error;
pub use errors::*;

#[cfg(test)]
mod test;


use std::{
    ptr::NonNull,
};

/// # Map of test type
/// the implementation of a AVl self balancing tree 
/// 
/// should add a little bit more info on the implementation
pub struct Map<KeyType:Ord, ContentType>{
    head: Option<MapLink<KeyType, ContentType>>,
    size: usize,
}


#[allow(dead_code)]
struct MapNode<KeyType:Ord, ContentType>{
    key: KeyType,
    content: ContentType,
    father: Option<MapLink<KeyType,ContentType>>,
    depth: structs::BinarySon<i32>,
    son: structs::BinarySon<Option<MapLink<KeyType,ContentType>>>,
    metadata: into_precompiled::FeatureField,
}

type MapLink<KeyType, ContentType> = NonNull<MapNode<KeyType, ContentType>>;

/*
/// # set
pub struct Set<KeyType:Ord>{
    //head: Option<SetLink<KeyType>>,
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
    
    

    /// This function returns a `Map<KeyType, ContentType>`
    ///
    /// The map will not allocate until elements are inserted/added.
    ///
    /// `KeyType` should implement Ord
    ///
    /// # Example
    /// 
    /// ```
    /// let avl:gavl::Map<String, i32> = gavl::Map::new();
    /// ```
    pub fn new() -> Self {
        Self{head:None ,size:0}
    }
    
    

    /// Inserts a node into the `Map`
    /// 
    /// # Examples
    /// ```
    /// let mut map = gavl::Map::<String, i32>::new();
    /// for elem in 0..10 {
    ///     map.add(elem.to_string(), elem);
    /// }
    /// assert_eq!(map.len(), 10);
    /// ```
    /// # Returns
    /// ## Success
    /// * `Ok(())`
    /// 
    /// ## Errors
    /// 
    /// * `Err(Error::KeyOcupied)`:   is returned if the key is already present 
    /// 
    pub fn add(&mut self, key:KeyType, content:ContentType) -> Result<(), Error> {
        let new_node = MapNode::new_map_link(key, content);
        
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                Ok(())
            }
            Some(data) => {
                if let Err(_place) = MapNode::insert_node(data, new_node) {
                    MapNode::free_node(new_node);
                    return Err(Error::KeyOcupied);
                }
                self.size += 1;
                self.compute_balance_additive(new_node);
                Ok(())
            }
        }
        
    }
    
    
    //// Inserts a node into the `Map` and replaces the content if it already exists returning old
    /// Replaces or adds node to `Map`
    /// * If the key is not present in the map it works just like `add`
    /// 
    /// * If it already present it will remplace the `content` with new one return
    /// the old value
    /// 
    ///
    /// # Examples
    /// ```
    /// let mut map = gavl::Map::<&str, i32>::new();
    /// const key:&str = "key";
    /// let old_content = map.insert(key, 12);
    /// let holder = map.get(&key);
    /// 
    /// assert_eq!(old_content, None);
    /// assert_eq!(holder, Ok(&12));
    /// 
    /// let old_content = map.insert(key, 13);
    /// let holder = map.get(&key);
    /// 
    /// assert_eq!(old_content, Some(12));
    /// assert_eq!(holder, Ok(&13));
    /// ```
    /// # Returns
    /// ## Success
    /// * `None`:   key didn't existed in `Map`
    /// * `Some(oldContent)`:   key's content was replaced
    pub fn insert(&mut self, key:KeyType, content:ContentType) -> Option<ContentType> {
        let new_node = MapNode::new_map_link(key, content);
        
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                None
            }
            Some(data) => {
                if let Err(place) = MapNode::insert_node(data, new_node) {
                    self.replace_node(place, new_node);
                    return Some(MapNode::unpack_node(place));// change to return the value (Error::KeyOcupied);
                }
                self.size += 1;
                self.compute_balance_additive(new_node);
                None
            }
            
        }
        
    }
    
    
    
    /*
    pub fn replace(&mut self, key:KeyType, content:ContentType) -> bool {
        let new_node = MapNode::new_map_link(key, content);
        
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                false
            }
            Some(data) => {
                if let Err(place) = MapNode::insert_node(data, new_node) {
                    self.replace_node(place, new_node);
                    MapNode::free_node(place);
                    return true;// change to return the value (Error::KeyOcupied);
                }
                self.size += 1;
                self.compute_balance_additive(new_node);
                false
            }
            
        }
        
    }
    */
    
    

    /// Deletes all the nodes in the `Map`
    /// will run 
    ///
    /// # Examples
    /// ```
    /// let mut map = gavl::Map::<String, i32>::new();
    /// for elem in 0..10 {
    ///     map.add(elem.to_string(), elem);
    /// }
    /// 
    /// assert_eq!(map.len(), 10);
    /// map.empty();
    /// assert_eq!(map.len(), 0);
    /// ```
    /// 
    pub fn empty(&mut self) {
        let empty_iter = self.empty_iter();
        for _elem in empty_iter {
            //just drop them
        }
    }
    
    

    pub fn get(&mut self, key:&KeyType) -> Result<&ContentType, Error> {
        let pivot = match self.head {
            None => {return Err(Error::NotFound);}
            Some(data) => data,
        };
        let node = MapNode::find_node(key, pivot).ok_or(Error::NotFound)?;
        let node_ref = unsafe{node.as_ref()};
        Ok(&node_ref.content)
    }
    
    
    
    pub fn get_mut(&mut self, key:&KeyType) -> Result<&mut ContentType, Error> {
        let pivot = match self.head {
            None => {return Err(Error::NotFound);}
            Some(data) => data,
        };
        let mut node = MapNode::find_node(key, pivot).ok_or(Error::NotFound)?;
        let node_mut = unsafe{node.as_mut()};
        Ok(&mut node_mut.content)
    }



    pub fn remove(&mut self, key:&KeyType) -> Result<ContentType, Error> {
        match self.size {
            0 => {
                Err(Error::NotFound)
            }
            1 => {
                let head = self.head.unwrap();
                let head_ref = unsafe{head.as_ref()};
                if !head_ref.key.cmp(key).is_eq() {
                    return Err(Error::NotFound);
                }
                self.size = 0;
                self.head = None;
                let target = unsafe{Box::from_raw(head.as_ptr())};
                Ok(target.content)
            }
            _ => {
                let target = MapNode::find_node(key, self.head.unwrap()).ok_or(Error::NotFound)?;
                let balance_pivot = self.compute_subtraccion_pivot(target);
                self.compute_balance_subtractive(balance_pivot);
                self.size -= 1;
                let target = unsafe{Box::from_raw(target.as_ptr())};
                Ok(target.content)
            }
        }
    }
    
    

    pub fn delete(&mut self, key:&KeyType) -> Result<(), Error> {
        match self.size {
            0 => {
                Err(Error::NotFound)
            }
            1 => {
                let head = self.head.unwrap();
                let head_ref = unsafe{head.as_ref()};
                if !head_ref.key.cmp(key).is_eq() {
                    return Err(Error::NotFound);
                }
                self.size = 0;
                self.head = None;
                MapNode::free_node(head);
                Ok(())
            }
            _ => {
                let target = MapNode::find_node(key, self.head.unwrap()).ok_or(Error::NotFound)?;
                let balance_pivot = self.compute_subtraccion_pivot(target);
                self.compute_balance_subtractive(balance_pivot);
                self.size -= 1;
                MapNode::free_node(target);
                Ok(())
            }
        }
    }
    
    

    /// Returns the number of elemnts in the `Map`
    ///
    /// # Examples
    /// ```
    /// let mut map = gavl::Map::<String, i32>::new();
    /// for elem in 0..10 {
    ///     map.add(elem.to_string(), elem);
    /// }
    /// assert_eq!(map.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    

}


