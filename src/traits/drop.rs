use super::{
    super::{
        Map,
    }
};

use std::ops::Drop;


impl<KeyType:Ord, ContentType> Drop for Map<KeyType, ContentType> {
    fn drop(&mut self) {
        println!("{:?} nodes left map was drop", self.len());
        self.empty();
        println!("{:?} nodes left map was drop", self.len());
    }
    
}


