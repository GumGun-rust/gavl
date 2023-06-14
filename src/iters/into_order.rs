use std::{
    iter::Iterator,
    fmt::Debug,
};

use super::{
    OrderIter,
    IterNode,
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
    
    pub fn into_order_iter(mut self) -> OrderIter<T, U> {
        self.calculate_indexes();
        OrderIter{
            started: false,
            current: self.head,
            holder: self,
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
    
}

impl<T:Ord, U> OrderIter<T, U> {
    
    
}

impl<T:Ord, U> Iterator for OrderIter<T, U> {
    type Item = IterNode<T, U>;
    
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

