use core::{
    cmp::{
        Ordering,
        max
    },
};

use super::{
    structs::{
        Side,
    },
    MapNode,
    Map,
    MapLink,
};

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType>{
    
    pub(super) fn compute_height(&mut self ,mut pivot: MapLink<KeyType, ContentType>) {
        
        loop{           
            let side = MapNode::get_side(pivot);
            let side = match side {
                Some(side) => {
                    side
                },
                None => {
                    break;
                }
            };
            
            let pivot_ref = unsafe {pivot.as_ref()};
            let mut pivot_father = pivot_ref.father.unwrap();
            let pivot_father_mut = unsafe {pivot_father.as_mut()};
            let pivot_new_depth = MapNode::get_max_height(pivot)+1;
            
            if pivot_father_mut.depth[side] >= pivot_new_depth {
                break;
            }
            pivot_father_mut.depth[side] = pivot_new_depth;
            
            let balance_factor = pivot_father_mut.depth[Side::Left] - pivot_father_mut.depth[Side::Right];
            //println!("pivot after balance factor {:?}", pivot_ref.content);
            if balance_factor >= 2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Left => {
                        self.rotate_right(pivot);
                    },
                    Side::Right => {
                        let pivot_mut = unsafe { pivot.as_ref() };
                        let pivot_son = pivot_mut.son[Side::Right].unwrap();
                        self.rotate_left(pivot_son);
                        self.rotate_right(pivot_son);
                    },
                }
            }
            if balance_factor <= -2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Right => {
                        self.rotate_left(pivot);
                    },
                    Side::Left => {
                        let pivot_mut = unsafe { pivot.as_ref() };
                        let pivot_son = pivot_mut.son[Side::Left].unwrap();
                        self.rotate_right(pivot_son);
                        self.rotate_left(pivot_son);
                    },
                }
            }
            
            pivot = pivot_father;
        }
    }
    
    
    fn rotate_right(&mut self, mut pivot:MapLink<KeyType, ContentType>) {
        let mut pivot_mut = unsafe{ pivot.as_mut() };
        let mut pivot_father = pivot_mut.father.unwrap();
        let mut pivot_father_mut = unsafe{ pivot_father.as_mut() };
        
        pivot_mut.father = pivot_father_mut.father;
        match pivot_father_mut.father {
            None => {
                self.head = Some(pivot);
            },
            Some(mut granfather) => {
                let father_side = MapNode::get_side(pivot_father).unwrap();
                let granfather_mut = unsafe { granfather.as_mut() };
                granfather_mut.son[father_side] = Some(pivot);
            }
        }
        
        pivot_father_mut.depth[Side::Left] = pivot_mut.depth[Side::Right];
        pivot_father_mut.son[Side::Left] = pivot_mut.son[Side::Right];
        if let Some(mut pivot_son) = pivot_mut.son[Side::Right] {
            let pivot_son_mut = unsafe{ pivot_son.as_mut() };
            pivot_son_mut.father = Some(pivot_father);
        }
        
        pivot_father_mut.father = Some(pivot);
        pivot_mut.son[Side::Right] = Some(pivot_father);
        pivot_mut.depth[Side::Right] = MapNode::get_max_height(pivot_father)+1;
    }
    
    fn rotate_left(&mut self, mut pivot:MapLink<KeyType, ContentType>) {
        let mut pivot_mut = unsafe{ pivot.as_mut() };
        let mut pivot_father = pivot_mut.father.unwrap();
        let mut pivot_father_mut = unsafe{ pivot_father.as_mut() };
        
        pivot_mut.father = pivot_father_mut.father;
        match pivot_father_mut.father {
            None => {
                self.head = Some(pivot);
            },
            Some(mut granfather) => {
                let father_side = MapNode::get_side(pivot_father).unwrap();
                let granfather_mut = unsafe { granfather.as_mut() };
                granfather_mut.son[father_side] = Some(pivot);
            }
        }
        
        pivot_father_mut.depth[Side::Right] = pivot_mut.depth[Side::Left];
        pivot_father_mut.son[Side::Right] = pivot_mut.son[Side::Left];
        if let Some(mut pivot_son) = pivot_mut.son[Side::Left] {
            let pivot_son_mut = unsafe{ pivot_son.as_mut() };
            pivot_son_mut.father = Some(pivot_father);
        }
        
        pivot_father_mut.father = Some(pivot);
        pivot_mut.son[Side::Left] = Some(pivot_father);
        pivot_mut.depth[Side::Left] = MapNode::get_max_height(pivot_father)+1;
    }
}


impl<KeyType:Ord, ContentType> MapNode<KeyType, ContentType>{
    
    
    pub(super) fn free_node(node: MapLink<KeyType, ContentType>) {
        //from NonNull to Box
        let _ = unsafe{Box::from_raw(node.as_ptr())};
        
        //todo!();
    }
    
    pub(super) fn insert_node(mut pivot: MapLink<KeyType, ContentType>, mut node: MapLink<KeyType, ContentType>) -> bool{
        loop{
            let key_order = unsafe{ node.as_ref().key.cmp(&pivot.as_ref().key) };
            let side = match key_order {
                Ordering::Less => {
                    Side::Left
                },
                Ordering::Equal => {
                    Self::free_node(node);
                    return false;
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
                    //pivot_mut.depth[side] = 1;
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

    fn get_max_height(node: MapLink<KeyType, ContentType>)-> i32 {
        let node_ref = unsafe {node.as_ref()};
        max(node_ref.depth[Side::Left], node_ref.depth[Side::Right])
    }
    
    pub(crate) fn get_side(node: MapLink<KeyType, ContentType>) -> Option<Side> {
        let node_father_ref = unsafe{node.as_ref().father?.as_ref()};
        if let Some(test) = node_father_ref.son[Side::Left] {
            if test == node {
                return Some(Side::Left);
            }
        }
        if let Some(test) = node_father_ref.son[Side::Right] {
            if test == node {
                return Some(Side::Right);
            }
        }
        None
    }

    fn get_deepest_son_side(node:MapLink<KeyType, ContentType>) -> Side {
        let node_ref = unsafe{ node.as_ref() };
        match node_ref.depth[Side::Left].cmp(&node_ref.depth[Side::Right]) {
            Ordering::Less => {
                Side::Right
            },
            Ordering::Greater => {
                Side::Left
            },
            Ordering::Equal => {
                panic!();
            },
            
        }
    }
}

