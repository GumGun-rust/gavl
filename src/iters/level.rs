use std::{
    iter::Iterator,
    marker::PhantomData,
    fmt::Debug,
};

use super::{
    LevelIter,
    //LevelIterMut,
    super::{
        structs::{
            Side,
        },
        Map,
        Node,
        Link
    },
};


impl<T:Ord+Debug, U> Map<T, U> {
    
    pub(super) fn set_indexes(&mut self) {
        let /*mut*/ index = 1u64;
        let /*mut*/ _level = 0u32;
        
        let mut pivot = match self.head {
            None => { return; },
            Some(/*mut*/pivot) => pivot,
        };
        
        let pivot_mut = unsafe{pivot.as_mut()};
        pivot_mut.index = index;
        //index += 1;
        
        loop {
            let control = Self::get_next_node(pivot);
            pivot = match control {
                Some(next) => next,
                None => {break;}
            };
            
            let control_ref = unsafe{control.unwrap().as_ref()};
            println!("--- {:?}", control_ref.key);
            
        }
        
    }
    
    fn get_next_node(pivot:Link<T, U>) -> Option<Link<T, U>> {
        let side = Node::get_side(pivot);
        match side {
            Some(posible_side) => {
                let node_ref = unsafe{pivot.as_ref()};
                println!("{:?}", node_ref.key);
                let father = node_ref.father.unwrap();
                let father_ref = unsafe{father.as_ref()};
                let mut level = 1;
                let /*mut*/ side;
                match posible_side {
                    Side::Left => {
                        father_ref.son[Side::Right]
                    },
                    Side::Right => {
                        let mut pivot = father;
                        loop {
                            side = Node::get_side(pivot);
                            level += 1;
                            match side {
                                Some(_posible_side) => {
                                    panic!();
                                    //logic when its not a direct road
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                        for _ in 0..level {
                            let pivot_ref = unsafe{pivot.as_ref()};
                            pivot = pivot_ref.son[Side::Left].unwrap();
                        }
                        Some(pivot)
                    }
                }
            }, 
            None => {
                
                println!("{:?}", side);
                let node_ref = unsafe{pivot.as_ref()};
                println!("{:?}", node_ref.key);
                node_ref.son[Side::Left]
            }
        }
    }
    
    pub fn level_iter(&mut self) -> LevelIter<T, U> {
        self.set_indexes();
        
        
        LevelIter{
            /*I
            state:2,
            data_struct: self,
            */
            current: self.head,
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
    
    /*
    pub fn level_iter_mut(&mut self) -> LevelIterMut<T, U> {
        LevelIterMut{
            state:2,
            data_struct: self,
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }
    */
}

impl<'a, T:Ord, U> Iterator for LevelIter<'a, T, U> {
    type Item = (&'a T, &'a U);
    
    fn next(&mut self) -> Option<Self::Item> {
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


