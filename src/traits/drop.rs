use super::{
    super::{
        Map,
    }
};

use std::ops::Drop;


impl<KeyType:Ord, ContentType> Drop for Map<KeyType, ContentType> {
    fn drop(&mut self) {
        self.empty();
    }
    
}


