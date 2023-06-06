use core::{
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
};

use super::{
    structs::Side,
    Node,
    Map,
    Link,
};

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
                }
                let _ = builder.field("LeftDepth", &&(depth[Side::Left]));
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
                let _ = builder.field("RightDepth", &&(depth[Side::Right]));
                if let Some(data) = son[Side::Right] {
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

#[allow(dead_code)]
fn dbg_prnt<T:Ord+Debug, U:Debug>(node:Link<T, U>) {
    let node_ref = unsafe{ node.as_ref() };
    println!("\n<><>\nnode {:?} content{:?}\n<><>\n", node, node_ref.content);
}

