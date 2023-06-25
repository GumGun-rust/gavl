use super::{
    structs::{
        BinarySon
    },
};

#[derive(Debug, Default)]
pub(crate) struct PrecompiledMetadata{
    pub index: usize,
    pub son_index: BinarySon<Option<usize>>,
}

#[derive(Debug)]
pub struct PrecompiledIterNode<KeyType:Ord, ContentType>{
    pub key: KeyType,
    pub content: ContentType,
    pub head: bool,
    pub prev_index: Option<usize>,
    pub next_index: Option<usize>,
}

impl<KeyType:Ord+Clone, ContentType:Clone> Clone for PrecompiledIterNode<KeyType, ContentType>{
    
    fn clone(&self) -> Self {
        Self{
            key:self.key.clone(),
            content:self.content.clone(),
            head:self.head.clone(),
            prev_index:self.prev_index.clone(),
            next_index:self.next_index.clone(),
        }
    }
}

impl<KeyType:Ord+Copy, ContentType:Copy> Copy for PrecompiledIterNode<KeyType, ContentType>{}

