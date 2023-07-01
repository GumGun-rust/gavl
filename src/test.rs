use super::*;


pub(crate) fn print_type_of<KeyType>(_: &KeyType) {
    println!("{}", std::any::type_name::<KeyType>())
}

mod avl_test{
    use super::*;
    
    #[test]
    fn individual_remove() {
        let mut avl = Map::<u64,u64>::new();
        avl.add(2, 69).unwrap();
        let remove_value = avl.remove(&2).unwrap();
        assert_eq!(remove_value, 69);
        assert_eq!(avl.len(), 0);
        avl.add(2, 69).unwrap();
        avl.remove(&4).unwrap_err();
        assert_eq!(avl.len(), 1);
    }
    
    #[test]
    fn individual_delete() {
        let mut avl = Map::<u64,u64>::new();
        avl.add(2, 100).unwrap();
        avl.delete(&2).unwrap();
        assert_eq!(avl.len(), 0);
        avl.add(2, 100).unwrap();
        avl.delete(&4).unwrap_err();
        assert_eq!(avl.len(), 1);
    }
    
    #[test]
    fn test_deletion_double_balance() { 
        let mut avl = Map::<u64,u64>::new();
        for elem in 4+0..4+15 {
            avl.add(elem*2, 100).unwrap();
        }
        avl.add(17, 100).unwrap();
        println!("{:#?}", &avl);
        avl.delete(&22).unwrap();
        println!("{:#?}", &avl);
    }
    
    #[test]
    fn test_deletion_simple_balance() { 
        let mut avl = Map::<u64,u64>::new();
        for elem in 4+0..4+15 {
            avl.add(elem*2, 100).unwrap();
        }
        avl.add(15, 100).unwrap();
        avl.delete(&22).unwrap();
        println!("{:#?}", &avl);
    }
    
    #[test]
    fn test_deletion_index_propagation() { 
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 4+0..4+15 {
            avl.add(elem*2, 100).unwrap();
        }
        avl.add(1, 0).unwrap();
        println!("{:#?}", &avl);
        avl.delete(&1).unwrap();
        println!("{:#?}", &avl);
    }
    
    #[test]
    fn test_deletion_interrupted_index_propagation() { 
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 4+0..4+31 {
            avl.add(elem*2, 100).unwrap();
        }
        avl.add(1, 0).unwrap();
        avl.add(13, 0).unwrap();
        println!("{:#?}", &avl);
        avl.delete(&1).unwrap();
        println!("{:#?}", &avl);
    }
    
    #[test]
    fn test_double_insert() {
        let mut avl = Map::<u64,u64>::new();
        for elem in 0..7 {
            avl.add(elem, 7-elem).unwrap();
        }
        let hola = avl.add(2, 12).unwrap_err();
        println!("{:#?}", hola);
    }

    #[test]
    fn test_find() { 
        let mut avl = Map::<u64,u64>::new();
        for elem in 0..7 {
            avl.add(elem, 7-elem).unwrap();
        }
        println!("{:#?}", &avl);
        avl.get(&2).unwrap();
        avl.get(&13).unwrap_err();
    }
    
    #[test]
    fn test_creation() { 
        let mut avl0 = Map::<u64,u64>::new();
        let mut avl1 = Map::<u64,u64>::new();
        for elem in 0..7 {
            avl0.add(elem, 6-elem).unwrap();
        }
        for elem in 0..7 {
            avl1.add(6-elem, elem).unwrap();
        }
        println!("{:#?}", &avl0);
        println!("{:#?}", &avl1);
    }

    #[test]
    fn empty() {
        let mut avl = Map::<u64,u64>::new();
        for elem in 4+0..4+7+5 {
            avl.add(elem, 0).unwrap();
        }
        println!("{:#?}", &avl);
        assert_eq!(avl.len(), 12);
        avl.empty();
        println!("{:#?}", &avl);
        assert_eq!(avl.len(), 0);
    }
    
    #[test]
    fn drop() {
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 4+0..4+7+5 {
            avl.add(elem, 0).unwrap();
        }
        
    }
    
}
