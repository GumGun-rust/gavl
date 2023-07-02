use super::{
    super::{
        Map
    }
};

impl<KeyType:Ord+Clone, ContentType:Clone> Clone for Map<KeyType, ContentType> {
    fn clone(&self) -> Map<KeyType, ContentType>{
        self.iter_ref().map(|(key_ref, content_ref)|(key_ref.clone(), content_ref.clone())).collect()
    }
}

