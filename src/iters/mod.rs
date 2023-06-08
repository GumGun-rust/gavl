pub mod level;

use super::{
    Map,
    Link,
};

use std::marker::PhantomData;

pub struct LevelIter<'a, T:Ord, U> {
    state: i8,
    data_struct: &'a Map<T, U>,
    current: Link<T, U>,
    phantom0: PhantomData<&'a T>,
    phantom1: PhantomData<&'a U>,
}

/*
pub struct LevelIterMut<'a, T:Ord, U> {
    state: i8,
    data_struct: &'a mut Map<T, U>,
    phantom0: PhantomData<&'a T>,
    phantom1: PhantomData<&'a U>,
}
*/
