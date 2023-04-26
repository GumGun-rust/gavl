//use std::ptr;
use std::ptr::NonNull;
use arrayvec::ArrayString;


use core::{
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
};


type Link<T, U> = Option<NonNull<Node<T, U>>>;

//#[derive(Debug)]
pub struct Map<T:Ord, U>{
    head: Link<T, U>,
    size: u64,
}

//#[derive(Debug)]
struct Node<T:Ord, U>{
    index: u64,
    key: T,
    content: U,
    
    
}

pub fn log(){
    /*
    let mut hasher = DefaultHasher::new();
    hasher.write_u32(1989);
    println!("Hash is {:x}!", hasher.finish());
    */
    let mut string:ArrayString<16> = ArrayString::<16>::new();
    
    string.push_str("foo");

    println!("hola {}", string);
}


impl<T:Ord, U> Map<T, U>{
    pub fn new() -> Self {
        Self{head:None ,size:0}
    }
    
    pub fn add(&mut self, key:T, content:U) -> bool {
        let new_node = NonNull::new(Box::into_raw(Box::new(Node{
            key,
            content,
            index:0,
        })));
        
        let _ = new_node.expect("system ran out of memory");
        
        //println!("{:?}", new_node);
        match self.head.as_ref() {
            None => {
                self.head = new_node;
                self.size = self.size+1;
                println!("no tiene datos");
                return false;
            },
            Some(data) => {
                println!("tiene datos");
                panic!();
            }
        }
        
    }
}

impl<T:Ord+Debug, U:Debug> Debug for Node<T, U>{
    fn fmt(&self, formater: &mut Formatter) -> fmt::Result {
        match *self {
            Node{
                index: _,
                ref key,
                ref content,
                
            } => {
            let mut builder = formater.debug_struct("Node");
            let _ = builder.field("key", &&(*key));
            let _ = builder.field("key", &&(*content));
            builder.finish()
            }
        }
    }
}

impl<T:Ord+Debug, U:Debug> Debug for Map<T, U>{
    fn fmt(&self, formater: &mut Formatter) -> fmt::Result {
        match *self {
            Map{
                ref size,
                ref head
            } => {
            let mut builder = formater.debug_struct("Map");
            let _ = builder.field("size", &&(*size));
            match head.as_ref() {
                None => {
                    let _ = builder.field("head", &&(*head));
                    
                },
                Some(node) => unsafe {
                    let _ = builder.field("head", &&((node.as_ref())));
                }
            }
            builder.finish()
            }
        }
    }
}

