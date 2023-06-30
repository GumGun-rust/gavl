            
            let balance_factor = pivot_mut.depth[Side::Left] - pivot_mut.depth[Side::Right];
            println!("{:?}", balance_factor);
            if balance_factor >= 2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Left => {
                        let pivot_mut = unsafe{pivot.as_ref()};
                        let pivot_son = pivot_mut.son[Side::Right].unwrap();
                        self.rotate_left(pivot_son);
                        self.rotate_right(pivot_son);
                        todo!();
                    },
                    Side::Right => {
                        let pivot_mut = unsafe{pivot.as_mut()};
                        let pivot_son = pivot_mut.son[Side::Left].unwrap();
                        self.rotate_right(pivot_son);
                    },
                }
                pivot = pivot_mut.father.unwrap();
                pivot_mut = unsafe{pivot.as_mut()};
            }
            if balance_factor <= -2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Right => {
                        let pivot_mut = unsafe{pivot.as_ref()};
                        let pivot_son = pivot_mut.son[Side::Left].unwrap();
                        self.rotate_right(pivot_son);
                        self.rotate_left(pivot_son);
                        todo!();
                    },
                    Side::Left => {
                        self.rotate_left(pivot);
                        todo!();
                    },
                }
                pivot = pivot_mut.father.unwrap();
            }
            
            let pivot_new_depth;
            
            match MapNode::get_side(pivot) {
                Some(side) => {
                    pivot = pivot_mut.father.unwrap();
                    pivot_new_depth = MapNode::get_max_height(pivot)+1;
                    
                    if pivot_mut.depth[side] >= pivot_new_depth {
                        break;
                    }
                    
                },
                None => {
                    break;
                }
            }
            
            pivot_mut = unsafe{pivot.as_mut()};
            
            

        
        /*
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
            println!("{:?}", balance_factor);
            
            if balance_factor >= 2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Left => {
                        let pivot_mut = unsafe { pivot.as_ref() };
                        let pivot_son = pivot_mut.son[Side::Right].unwrap();
                        self.rotate_left(pivot_son);
                        self.rotate_right(pivot_son);
                    },
                    Side::Right => {
                        self.rotate_right(pivot);
                    },
                }
            }
            if balance_factor <= -2 {
                match MapNode::get_deepest_son_side(pivot) {
                    Side::Right => {
                        let pivot_mut = unsafe { pivot.as_ref() };
                        let pivot_son = pivot_mut.son[Side::Left].unwrap();
                        self.rotate_right(pivot_son);
                        self.rotate_left(pivot_son);
                    },
                    Side::Left => {
                        self.rotate_left(pivot);
                    },
                }
            }
            
            pivot = pivot_father;
            side_holder = MapNode::get_side(pivot);
        }
        */
        
