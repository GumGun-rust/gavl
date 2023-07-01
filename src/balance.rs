use std::{
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
    
    pub(super) fn compute_balance_additive(&mut self, mut pivot:MapLink<KeyType, ContentType>) {
        let mut side_holder = MapNode::get_side(pivot);
        //println!("{:?}", pivot);
        while let Some(side) = side_holder {           
            let pivot_ref = unsafe {pivot.as_ref()};
            let mut pivot_father = pivot_ref.father.unwrap();
            let pivot_father_mut = unsafe {pivot_father.as_mut()};
            let pivot_new_depth = MapNode::get_max_height(pivot)+1;
            
            if pivot_father_mut.depth[side] >= pivot_new_depth {
                break;
            }
            pivot_father_mut.depth[side] = pivot_new_depth;
            
            let balance_factor = pivot_father_mut.depth[Side::Left] - pivot_father_mut.depth[Side::Right];
            //println!("{:?}", balance_factor);
            
            if balance_factor >= 2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Left => {
                        self.rotate_right(pivot);
                    },
                    Side::Right => {
                        let pivot_mut = unsafe{pivot.as_ref()};
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
                        let pivot_mut = unsafe{pivot.as_ref()};
                        let pivot_son = pivot_mut.son[Side::Left].unwrap();
                        self.rotate_right(pivot_son);
                        self.rotate_left(pivot_son);
                    },
                }
            }
            
            pivot = pivot_father;
            side_holder = MapNode::get_side(pivot);
        }
    }
    
    

    pub(super) fn compute_balance_subtractive(&mut self, mut pivot:MapLink<KeyType, ContentType>) {
        println!("-------------------------------------------");
        println!("pivot ====== {:?}", pivot);
        let mut pivot_mut = unsafe{pivot.as_mut()};
        let mut last_side:Option<Side> = None;
        
        loop {
            match pivot_mut.get_balance_factor() {
                Some(balance_side) => {
                    let pivot_ref_tmp = unsafe{pivot.as_ref()};
                    let mut pivot_son_tmp = pivot_ref_tmp.son[balance_side].unwrap();
                    let pivot_son_ref_tmp = unsafe{pivot_son_tmp.as_mut()};
                    
                    match MapNode::get_deepest_son_side(pivot_son_tmp) {
                        gs_side if gs_side == balance_side => {
                            println!("\n\n\t\t\tbalance_side {:?}\n\t\t\tgs_side {:?}", balance_side, gs_side);
                            println!("\t\t\tson {:?}\n\n", pivot_son_tmp);
                            self.variable_rotate(pivot_son_tmp, balance_side.complement());
                            //panic!("up");
                            //println!("son\t\t{:?}\ngranson\t\t{:?}", pivot_son_tmp, pivot_granson_tmp);
                            
                        },
                        gs_side @ _ => {
                            println!("\n\n\t\t\tbalance_side {:?}\n\t\t\tgs_side {:?}\n\n", balance_side, gs_side);
                            println!("\t\t\tson {:?}\n\n", pivot_son_tmp);
                            let pivot_granson_tmp = pivot_son_ref_tmp.son[gs_side].unwrap();
                            self.variable_rotate(pivot_granson_tmp, balance_side);
                            self.variable_rotate(pivot_granson_tmp, gs_side);
                            /*
                            println!("-----\n\t\t{:?}\n", gs_side);
                            self.variable_rotate(pivot_son_holder, gs_side);
                            */
                        },
                    }
                    pivot = pivot_mut.father.unwrap();
                    pivot_mut = unsafe{pivot.as_mut()};
                },
                None => {}
            }
            
            
            //println!("last side {:?}", last_side);
            if let Some(old_side) = last_side {
                println!("loop");
                //println!("{:?}", old_side);
                let test = pivot_mut.son[old_side].unwrap();
                //println!("{:?}", test);
                let pivot_new_depth = MapNode::get_max_height(test)+1;
                
                if pivot_mut.depth[old_side] <= pivot_new_depth {
                    println!("se acabaron los cambios");
                    return;
                }
                //println!("se cambio");
                pivot_mut.depth[old_side] = pivot_new_depth;
                //pivot_mut
            }
            
            let next_pivot = match pivot_mut.father {
                Some(next_pivot) => {
                    //println!("nextPiv {:?}", next_pivot);
                    next_pivot
                },
                None => {
                    //println!("break");
                    break;
                }
            };
            
            last_side = MapNode::get_side(pivot);
            pivot = next_pivot;
            pivot_mut = unsafe{pivot.as_mut()};
        }
    }

    
    
    pub(super) fn compute_subtraccion_pivot(&mut self, target:MapLink<KeyType, ContentType>) -> MapLink<KeyType, ContentType> {
        let tmp_holder = MapNode::get_replacement(target);
        println!("{:?}", tmp_holder);
        match tmp_holder {
            Some(pivot) => {
                let pivot_ref = unsafe{pivot.as_ref()};
                let pivot_side = MapNode::get_side(pivot).expect("should be granted to have father");
                let holder = match pivot_ref.son[pivot_side.complement()] {
                    Some(mut pivot_son) => {
                        let side = MapNode::get_side(pivot).unwrap();
                        let pivot_son_mut = unsafe{pivot_son.as_mut()};
                        let father = pivot_ref.father;
                        let father_mut =  unsafe{father.unwrap().as_mut()};
                        
                        pivot_son_mut.father = father;
                        father_mut.son[side] = Some(pivot_son);
                        println!("--pivot_son--{:?}", pivot_son);
                        pivot_son
                    },
                    None => {
                        let mut father = pivot_ref.father.unwrap();
                        let father_mut = unsafe{father.as_mut()};
                        father_mut.son[pivot_side] = None;
                        father_mut.depth[pivot_side] = 0;
                        println!("--father--{:?}", father);
                        father
                    },
                };
                self.replace_node(target, pivot);
                holder
            },
            None => {
                let side = MapNode::get_side(target).expect("should be granted to have father");
                let target_ref = unsafe{target.as_ref()};
                let mut father = target_ref.father.unwrap();
                let father_mut = unsafe{father.as_mut()};
                father_mut.son[side] = None;
                father_mut.depth[side] = 0;
                father
            }
        }
    }
    
    
    fn variable_rotate(&mut self, pivot:MapLink<KeyType, ContentType>, side:Side) {
        match side {
            Side::Left => {
                self.rotate_left(pivot);
            },
            Side::Right => {
                self.rotate_right(pivot);
            }
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
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\n");
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\n");
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\n");
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
    


    //makes source take the place of dest in the tree
    fn replace_node(&mut self, dest:MapLink<KeyType, ContentType>, mut src:MapLink<KeyType, ContentType>) {
        let dest_ref = unsafe{dest.as_ref()};
        let src_mut = unsafe{src.as_mut()};
        src_mut.son = dest_ref.son;
        src_mut.depth = dest_ref.depth;
        src_mut.father = dest_ref.father;
        
        for side in [Side::Left, Side::Right] {
            if let Some(mut son) = src_mut.son[side] {
                let son_mut = unsafe{son.as_mut()};
                son_mut.father = Some(src);
            }
        }
        
        match MapNode::get_side(dest) {
            None => {
                self.head = Some(src);
            },
            Some(side) => {
                let father_mut = unsafe{src_mut.father.unwrap().as_mut()};
                father_mut.son[side] = Some(src);
            }
        }
        
    }

    

}


impl<KeyType:Ord, ContentType> MapNode<KeyType, ContentType>{
    
    

    pub(super) fn find_node(key:&KeyType, mut pivot:MapLink<KeyType, ContentType>) -> Option<MapLink<KeyType, ContentType>> { 
        loop {
            let pivot_ref = unsafe{pivot.as_ref()};
            let side;
            match key.cmp(&pivot_ref.key) {
                order @ (Ordering::Less|Ordering::Greater) => {
                    side = Side::try_from(order).unwrap();
                    match pivot_ref.son[side] {
                        None => {
                            return None;
                        },
                        Some(next_pivot) => {
                            pivot = next_pivot;
                        },
                    }
                },
                Ordering::Equal => {
                    break;
                },
            }
        }
        Some(pivot)
    }
    
    

    pub(super) fn free_node(node:MapLink<KeyType, ContentType>) {
        //from NonNull to Box
        //make it easyer to count the amount of dropped nodes
        let _ = unsafe{Box::from_raw(node.as_ptr())};
    }
    
    
    
    pub(super) fn insert_node(mut pivot:MapLink<KeyType, ContentType>, mut node:MapLink<KeyType, ContentType>) -> Result<(), ()>{
        loop{
            let key_order = unsafe{ node.as_ref().key.cmp(&pivot.as_ref().key) };
            
            let side = match Side::try_from(key_order) {
                Ok(side) => side,
                Err(_) => {
                    Self::free_node(node);
                    return Err(());
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
        Ok(())
    }
    
    

    fn get_replacement(node:MapLink<KeyType, ContentType>) -> Option<MapLink<KeyType, ContentType>> {
        let node_mut = unsafe{node.as_ref()};
        for son in [Side::Left, Side::Right] {
            let son_complement = son.complement();
            if let Some(mut pivot) = node_mut.son[son] {
                loop {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    match pivot_ref.son[son_complement] {
                        None => {break;},
                        Some(next_pivot) => {
                            pivot = next_pivot;
                        }
                    }
                }
                return Some(pivot);
            }
        }
        None
    }
    


    fn get_max_height(node:MapLink<KeyType, ContentType>)-> i32 {
        let node_ref = unsafe {node.as_ref()};
        max(node_ref.depth[Side::Left], node_ref.depth[Side::Right])
    }
    
    

    pub(super) fn get_side(node:MapLink<KeyType, ContentType>) -> Option<Side> {
        //one liner to get the reference of father or return None if node has no father
        let node_father_ref = unsafe{node.as_ref().father?.as_ref()};
        for side in [Side::Left, Side::Right] {
            if let Some(test) = node_father_ref.son[side] {
                if test == node {
                    return Some(side);
                }
            }
        }
        None
    }

    

    fn get_deepest_son_side(node:MapLink<KeyType, ContentType>) -> Side {
        let node_ref = unsafe{ node.as_ref() };
        let side_comparison = node_ref.depth[Side::Right].cmp(&node_ref.depth[Side::Left]);
        //should only arrive here when |balance_factor| >= 2 
        Side::try_from(side_comparison).expect("one should be bigger")
    }
    
    

    fn get_balance_factor(&self) -> Option<Side> {
        let balance_factor = self.depth[Side::Right] - self.depth[Side::Left];
        match balance_factor {
            -2 => Some(Side::Left),
            2 => Some(Side::Right),
            _ => None,
        }
    }
}

