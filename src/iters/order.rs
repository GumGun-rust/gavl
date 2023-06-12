use std::{
    //iter::Iterator,
    marker::PhantomData,
    fmt::Debug,
};

use super::{
    OrderIterRef,
    //LevelIter,
    super::{
        structs::{
            Side,
        },
        Map,
        Node,
        Link
    },
};

impl<T:Ord+Debug, U:Debug> Map<T, U> {
    pub fn order_iter_ref(&mut self) -> OrderIterRef<T, U> {
        self.calculate_indexes();
        OrderIterRef{
            started: false,
            current: self.head,
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
    
    fn calculate_indexes(&mut self) {
        if self.size == 0 {
            return;
        }
        
        let mut pivot = self.head.unwrap();
        loop {
            let pivot_ref = unsafe{pivot.as_ref()};
            match pivot_ref.son[Side::Left] {
                Some(son) => {
                    pivot = son;
                },
                None => {
                    break;
                }
            }
        }
        let pivot_mut = unsafe{pivot.as_mut()};
        pivot_mut.index = 0;
        let mut iter = 1;
        
        loop {
            match Self::next_node(pivot) {
                Some(mut node) => {
                    let node_mut = unsafe {node.as_mut()};
                    node_mut.index = iter;
                    pivot = node;
                    println!("{:#?} {}", node_mut.key, iter);
                },
                None => {
                    break;
                }
            }
            iter += 1;
        }
    }
    
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

impl<'a, T:Ord, U> OrderIterRef<'a, T, U> {
    
}

impl<'a, T:Ord, U> Iterator for OrderIterRef<'a, T, U> {
    type Item = (&'a T, &'a U);
    
    fn next(&mut self) -> Option<Self::Item> {
        None
        /*
        let _holder = self.current;
        match self.current {
            Some(node) => {
                let node_mut = unsafe{node.as_ref()};
                self.current = node_mut.son[Side::Left];
                return Some((&node_mut.key, &node_mut.content));
            },
            None => {
                return None;
            }
        }
        */
        
        
        /*
        if self.state != 0 {
            self.state -= 1;
            return Some(i32::from(self.state));
        }
        None
        //panic!();
        */
    }
}


#[test]
fn test() {
    let mut avl = Map::<u64,u64>::new();
    for number in 4+0..4+7 {
        avl.add(number, 0).unwrap();
    }
    //println!("{:#?}", avl);
    let iter_level = avl.order_iter_ref();//.enumerate();
    
    /*
    for elem in iter_level {
        println!("{:?}", elem);
    }
    */
    panic!();
    
}

