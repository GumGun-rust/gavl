#[allow(unused_imports)]
pub use precomputed_feature::*;

#[cfg(all(not(feature = "into_precomputed"), not(doc)))]
mod precomputed_feature {
    
    #[derive(Debug, Default)]
    pub(crate) struct FeatureField();
    
}

#[cfg(any(feature = "into_precomputed", doc))]
mod precomputed_feature {
    use super::{
        super::{
            structs::{
                BinarySon
            },
        }
    };
    
    #[derive(Debug, Default)]
    pub(crate) struct FeatureField{
        pub index: usize,
        pub son_index: BinarySon<Option<usize>>,
    }
    
    
    /// # Dependant on feature into_precomputed 
    /// Maybe one day I will change it to into_precomputed
    /// 
    /// This is the data structure returned by the `.into_iter_precomputed()` this saved to an
    /// array will directly give a structure ready for binary search
    #[derive(Debug)]
    pub struct PrecomputedIterNode<KeyType:Ord, ContentType>{
        pub key: KeyType,
        pub content: ContentType,
        pub head: bool,
        pub prev_index: Option<usize>,
        pub next_index: Option<usize>,
    }

    impl<KeyType:Ord+Clone, ContentType:Clone> Clone for PrecomputedIterNode<KeyType, ContentType>{
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

    impl<KeyType:Ord+Copy, ContentType:Copy> Copy for PrecomputedIterNode<KeyType, ContentType>{}

    
}
