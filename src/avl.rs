use std::ptr;

use std::ptr::NonNull;
use arrayvec::ArrayString;

type Link<T, U> = Option<NonNull<Node<T, U>>>;

#[derive(Debug)]
pub struct Map<T: Ord, U>{
    head: Link<T, U>,
    size: u64,
}

#[derive(Debug)]
struct Node<T:Ord, U>{
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
        }))).expect("system ran out of memory");
        
        println!("{:?}", new_node);
        match self.head.as_ref() {
            None => {
                
                println!("no tiene datos");
            },
            Some(data) => {
                println!("tiene datos");
            }
        }
        
        
        panic!();
    }
}
