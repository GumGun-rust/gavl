use core::{
    cmp::{
        Ordering,
        max
    },
    ops::{
        Index,
        IndexMut,
    },
    ptr::NonNull,
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
};

use crate::structs::Side;
use crate::structs::BinarySon;


type Link<T, U> = NonNull<Node<T, U>>;

pub struct Map<T:Ord, U>{
    head: Option<Link<T, U>>,
    size: u64,
}

struct Node<T:Ord, U>{
    index: u64,
    key: T,
    content: U,
    father: Option<Link<T,U>>,
    depth: BinarySon<u64>,
    son: BinarySon<Option<Link<T,U>>>,
}

pub fn log(){
    use arrayvec::ArrayString;

    /*
    let mut hasher = DefaultHasher::new();
    hasher.write_u32(1989);
    println!("Hash is {:x}!", hasher.finish());
    */
    let mut string:ArrayString<16> = ArrayString::<16>::new();
    
    string.push_str("foo");

    println!("hola {}", string);
}


impl<T:Ord+Debug, U:Debug> Map<T, U>{
    
    pub fn new() -> Self {
        Self{head:None ,size:0}
    }
    
    pub fn add(&mut self, key:T, content:U) -> bool {
        let new_node = NonNull::new(Box::into_raw(Box::new(Node{
            key,
            content,
            index:0,
            father:None,
            depth: BinarySon::default(),
            son: BinarySon::default(),
        }))).expect("system ran out of memory");
        
        //println!("{:?}", new_node);
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size = self.size+1;
                println!("insert on empty");
                return false;
            },
            Some(data) => {
                insert_node(data, new_node);
                //println!("{:?}", Node::get_side(new_node));
                Node::compute_height(new_node);
                return false;
                //panic!();
            }
        }
        
    }
    
}

fn insert_node<T:Ord+Debug, U:Debug>(mut pivot: Link<T, U>, mut node: Link<T, U>) -> bool{
    loop{
        let key_order = unsafe{ node.as_ref().key.cmp(&pivot.as_ref().key) };
        let side = match key_order {
            Ordering::Less => {
                Side::Left
            },
            Ordering::Equal => {
                panic!();
            },
            Ordering::Greater => {
                Side::Right
            }
        };
        let pivot_mut = unsafe { pivot.as_mut() };
        match pivot_mut.son[side] {
            None => {
                let node_mut = unsafe{ node.as_mut() };
                node_mut.father = Some(pivot);
                pivot_mut.son[side] = Some(node);
                pivot_mut.depth[side] = 1;
                //println!("last");
                break;
                //return true;
            },
            Some(next_pivot_pointer) => {
                pivot = next_pivot_pointer;
                //println!("non last");
            }
        }
        //println!("{:?}", side);
    }
    true
}


fn balance_tree<T:Ord+Debug, U:Debug>(pivot:Link<T,U>) -> bool {
    panic!();
}

fn rotation() -> bool {
    panic!();
}

impl<T:Ord+Debug, U:Debug> Node<T, U>{
    
    fn compute_height(mut pivot: Link<T, U>) -> bool {
        println!("computing height-------------------------");
        
        loop{           
            let side = Node::get_side(pivot);
            let side = match side {
                Some(side) => {
                    side
                },
                None => {
                    panic!("Es la raiz");
                }
            };
            
            let mut pivot_father = unsafe {pivot.as_ref().father.unwrap()};
            let pivot_father_mut = unsafe {pivot_father.as_mut()};
            let pivot_new_depth = Node::get_max_height(pivot)+1;
            
            println!("{:?}", side);
            println!("{:?}", pivot_father_mut);
            
            if pivot_father_mut.depth[side] >= pivot_new_depth {
                break;
            }
            pivot_father_mut.depth[side] = pivot_new_depth;
            pivot = pivot_father;
            
            //panic!("testPanic");
        }
        println!("computing height-------------------------\n\n");
        true
    }
    
    fn get_max_height(node: Link<T, U>)-> u64 {
        let node_ref = unsafe {node.as_ref()};
        max(node_ref.depth[Side::Left], node_ref.depth[Side::Right])
    }
    
    fn get_side(node: Link<T, U>) -> Option<Side> {
        let node_father_ref = unsafe{node.as_ref().father?.as_ref()};
        if let Some(test) = node_father_ref.son[Side::Left] {
            if test == node {
                unsafe {
                    println!("{:?} {:?} Left", test, node.as_ref());
                }
                println!("{:?} {:?} Left", test, node);
                return Some(Side::Left);
            }
        }
        if let Some(test) = node_father_ref.son[Side::Right] {
            if test == node {
                unsafe {
                    println!("{:?} {:?} Right", test, node.as_ref());
                }
                println!("{:?} {:?} Right", test, node);
                return Some(Side::Right);
            }
        }
        None
    }
}



impl<T:Ord+Debug, U:Debug> Debug for Node<T, U>{
    fn fmt(&self, formater: &mut Formatter) -> fmt::Result {
        match *self {
            Node{
                index:_,
                ref father,
                ref key,
                ref content, 
                ref depth, 
                ref son,
            } => {
                let mut builder = formater.debug_struct("Node");
                if let Some(data) = son[Side::Left] {
                    unsafe {
                        let _ = builder.field("LeftSon", &&(data.as_ref()));
                    }
                    let _ = builder.field("LeftDepth", &&(depth[Side::Left]));
                }
                match father {
                    Some(data) => {
                        let _ = builder.field("father", &&(*data));
                    },
                    None => {
                        let _ = builder.field("father", &&(*father));
                    }
                }
                let _ = builder.field("key", &&(*key));
                let _ = builder.field("content", &&(*content));
                if let Some(data) = son[Side::Right] {
                    let _ = builder.field("RightDepth", &&(depth[Side::Right]));
                    unsafe {
                        let _ = builder.field("RightSon", &&(data.as_ref()));
                    }
                }
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

