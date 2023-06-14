pub mod level;
pub mod order;
pub mod into_order;

use super::{
    structs::{
        Side,
    },
    Map,
    Node,
    Link,
};

use std::marker::PhantomData;

pub struct LevelIter<'a, T:Ord, U> {
    //state: i8,
    //data_struct: &'a Map<T, U>,
    current: Option<Link<T, U>>,
    phantom0: PhantomData<&'a T>,
    phantom1: PhantomData<&'a U>,
}

pub enum OrderIterRefEnum<'a, T:Ord, U> {
    newOrderIter(Map<T, U>),
    OrderIter1{
        current: Option<Link<T, U>>,
        phantom0: PhantomData<&'a T>,
        phantom1: PhantomData<&'a U>,
    }
}

pub struct OrderIterRef<'a, T:Ord, U> {
    started: bool,
    current: Option<Link<T, U>>,
    phantom0: PhantomData<&'a T>,
    phantom1: PhantomData<&'a U>,
}

pub struct OrderIter<T:Ord, U> {
    started: bool,
    current: Option<Link<T, U>>,
    holder: Map<T, U>,
}

pub struct IterNode<T:Ord, U> {
    key: T,
    content: U,
    left_index: Option<usize>,
    right_index: Option<usize>,
}

/*
pub struct LevelIterMut<'a, T:Ord, U> {
    state: i8,
    data_struct: &'a mut Map<T, U>,
    phantom0: PhantomData<&'a T>,
    phantom1: PhantomData<&'a U>,
}
*/

impl<T:Ord, U> Map<T, U> {
    
    fn next_node(current:Link<T, U>) -> Option<Link<T, U>> {
        let current_ref = unsafe{current.as_ref()};
        if let Some(mut pivot) = current_ref.son[Side::Right] {
            loop {
                let pivot_ref = unsafe{pivot.as_ref()};
                match pivot_ref.son[Side::Left] {
                    Some(son) => {
                        pivot = son;
                    },
                    None => {
                        return Some(pivot);
                    }
                }
            }
        }
        let mut pivot = current;
        loop {
            let side = Node::get_side(pivot);
            match side {
                Some(Side::Left) => {
                    return unsafe{pivot.as_ref()}.father;
                },
                Some(Side::Right) => {
                    pivot = unsafe{pivot.as_ref()}.father.unwrap();
                },
                None => {
                    return None;
                }
            }
        }
    }
}

