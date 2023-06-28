
impl<KeyType:Ord+Debug, ContentType> Iterator for IntoIter<KeyType, ContentType> {
    type Item = (KeyType, ContentType);
    
    fn next(&mut self) -> Option<Self::Item> {
        
        match self.iter_data {
            IntoIterEnum::NewIter => {
                let mut pivot = match self.map.head {
                    Some(head) => head,
                    None => {return None;}
                };
                self.map.head = None;
                loop{
                    let pivot_ref = unsafe{pivot.as_ref()};
                    match pivot_ref.son[Side::Left] {
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        },
                        None => {
                            break;
                        }
                    }
                }
                let holder = pivot;
                let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                let next = match holder_box.son[Side::Right] {
                    Some(mut next) => {
                        let next_mut = unsafe{next.as_mut()};
                        if let Some(mut father) = holder_box.father {
                            let father_mut = unsafe{father.as_mut()};
                            father_mut.son[Side::Left] = Some(next);
                            next_mut.father = Some(father);
                        } else {
                            next_mut.father = None;
                        }
                        Some(next)
                    },
                    None => {
                        if let Some(mut father) = holder_box.father {
                            let father_mut = unsafe{father.as_mut()};
                            father_mut.son[Side::Left] = None;
                        }
                        holder_box.father
                    },
                };
                self.iter_data = IntoIterEnum::Iter{next:next, phantom0:PhantomData, phantom1:PhantomData};
                Some((holder_box.key, holder_box.content))
            },
            IntoIterEnum::Iter{
                ref mut next,
                ..
            } => {
                match next {
                    Some(holder) => {
                        let holder_box = unsafe{Box::from_raw(holder.as_ptr())};
                        *next = match holder_box.son[Side::Right] {
                            Some(mut pivot) => {
                                let mut pivot_mut = unsafe{pivot.as_mut()};
                                if let Some(mut father) = holder_box.father {
                                    let father_mut = unsafe{father.as_mut()};
                                    father_mut.son[Side::Left] = Some(pivot);
                                    pivot_mut.father = Some(father);
                                } else {
                                    pivot_mut.father = None;
                                }
                                
                                loop {
                                    let pivot_mut = unsafe{pivot.as_mut()};
                                    match pivot_mut.son[Side::Left] {
                                        Some(next_pivot) => {
                                            pivot = next_pivot;
                                        },
                                        None => {
                                            break;
                                        }
                                    }
                                }
                                
                                Some(pivot)
                                //panic!();
                            },
                            None => {
                                if let Some(mut father) = holder_box.father {
                                    let father_mut = unsafe{father.as_mut()};
                                    father_mut.son[Side::Left] = None;
                                }
                                //panic!();
                                holder_box.father
                            },
                        };
                        Some((holder_box.key, holder_box.content))
                    },
                    None => {
                        None
                    },
                } 
                
            }
        }
    }
}
