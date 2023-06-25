use super::{
    super::{
        Map
    }
};

impl<KeyType:Ord+Clone, ContentType:Clone> Clone for Map<KeyType, ContentType> {
    fn clone(&self) -> Map<KeyType, ContentType>{
        panic!();
    }
}

