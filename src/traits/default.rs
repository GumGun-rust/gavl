use super::{
    super::{
        Map
    }
};

impl<KeyType:Ord, ContentType> Default for Map<KeyType, ContentType> {
    fn default() -> Map<KeyType, ContentType>{
        Self{head:None ,size:0}
    }
}

