mod structs;
mod test;
mod debug;
mod balance;
mod traits;

use core::{
    ptr::NonNull,
};


pub struct Map<T:Ord, U>{
    head: Option<Link<T, U>>,
    size: u64,
}

pub struct Node<T:Ord, U>{
    index: u64,
    key: T,
    content: U,
    father: Option<Link<T,U>>,
    depth: structs::BinarySon<i32>,
    son: structs::BinarySon<Option<Link<T,U>>>,
}

type Link<T, U> = NonNull<Node<T, U>>;

fn log() {
    println!("log");
}

impl<T:Ord, U> Map<T, U>{
    
    pub fn new() -> Self {
        traits::test();
        Self{head:None ,size:0}
    }
    
    pub fn add(&mut self, key:T, content:U) -> Result<(), ()> {
        let new_node = NonNull::new(Box::into_raw(Box::new(Node{
            key,
            content,
            index:0,
            father:None,
            depth: structs::BinarySon::default(),
            son: structs::BinarySon::default(),
        }))).expect("system ran out of memory");
        
        //println!("{:?}", new_node);
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = 1;
                Ok(())
            },
            Some(data) => {
                if !Node::insert_node(data, new_node) {
                    return Err(());
                }
                self.size += 1;
                self.compute_height(new_node);
                Ok(())
            }
        }
        
    }
}


