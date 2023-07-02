use super::{
    super::{
        Map
    }
};

impl<KeyType:Ord, ContentType> FromIterator<(KeyType, ContentType)> for Map<KeyType, ContentType> {
    fn from_iter<IterType:IntoIterator<Item=(KeyType, ContentType)>>(iter:IterType) -> Self {
        let mut holder:Map<KeyType, ContentType> = Map::new();
        for elem in iter {
            let _ = holder.add(elem.0, elem.1);
        }
        holder
    }
}
