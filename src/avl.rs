use arrayvec::ArrayString;
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

type Link<T, U> = NonNull<Node<T, U>>;

#[derive(Debug, Clone, Copy)]
enum Side{
    Left,
    Right
}

struct BinarySon<T>{
    content:[T; 2]
}

impl<T> Index<Side> for BinarySon<T>{
    type Output = T;
    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self.content[0],
            Side::Right => &self.content[1],
        }
    }
}

impl<T> IndexMut<Side> for BinarySon<T>{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Left => &mut self.content[0],
            Side::Right => &mut self.content[1],
        }
    }
}


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
            depth: BinarySon{content:[0,0]},
            son: BinarySon{content:[None,None]},
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
    unsafe{
        loop{
            let key_order = node.as_ref().key.cmp(&pivot.as_ref().key);
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
            let pivot_mut = pivot.as_mut();
            match pivot_mut.son[side] {
                None => {
                    unsafe {
                        let node_mut = node.as_mut();
                        node_mut.father = Some(pivot);
                    }
                    pivot_mut.son[side] = Some(node);
                    //println!("last");
                    break;
                },
                Some(data) => {
                    pivot = data;
                    //println!("non last");
                }
            }
            //println!("{:?}", side);
        }
        true
    }
}


fn balance_tree() -> bool {
    panic!();
}

fn rotation() -> bool {
    panic!();
}

impl<T:Ord+Debug, U:Debug> Node<T, U>{
    
    fn compute_height(mut pivot: Link<T, U>) -> bool {
        println!("computing height-------------------------\n\n");
        loop{           
            let side = Node::get_side(pivot);
            let side = match side {
                Some(side) => {
                    side
                },
                None => {
                    /*
                    codigo cuando es el papa
                    */
                    //None::<u8>.expect("es la raiz");
                    break;
                }
            };
            
            let mut pivot_father = unsafe {pivot.as_ref().father.unwrap()};
            let mut pivot_father_mut = unsafe {pivot_father.as_mut()};
            let pivot_new_depht = Node::get_max_height(pivot)+1;
            
            println!("{:?}", side);
            println!("{:?}", pivot_father_mut);
            //println!("{:?}", pivot_father);
            //println!("{:?}", pivot);
            //println!("{:?}", pivot_new_depht);
            
            if pivot_father_mut.depth[side] >= pivot_new_depht {
                
                /*
                dbg!(side);
                dbg!(pivot_father_mut.depth[side]);
                dbg!(pivot_new_depht);
                println!("no hay cambio {:?}", side);
                */
                break;
            }
            pivot_father_mut.depth[side] = pivot_new_depht;
            pivot = pivot_father;
            
            //println!("{:#?}",pivot_father.as_ref());
            //println!("{:?}",pivot);
            //println!("{:#?}",pivot.as_ref());
            //break;
        }
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

