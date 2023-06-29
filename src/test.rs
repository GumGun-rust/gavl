use super::*;


pub(crate) fn print_type_of<KeyType>(_: &KeyType) {
    println!("{}", std::any::type_name::<KeyType>())
}

mod avl_test{
    use super::*;
    
    //#[ignore]
    #[test]
    fn test_find(){ 
        let mut avl = Map::<u64,u64>::new();
        
        for elem in 0..7 {
            avl.add(elem, 7-elem).unwrap();
        }
        println!("{:#?}", &avl);
        avl.get(&2).unwrap();
        avl.get(&13).unwrap_err();
    }

    #[test]
    fn test(){
        //avl::log();
        let mut hola = Map::<u64,u64>::new();
        //println!("{:#?}",hola);
        let _ = hola.add(3, 2);
        let _ = hola.add(4, 2);
        let _ = hola.add(5, 2);
        let _ = hola.add(6, 2);
        let _ = hola.add(7, 2);
        let _ = hola.add(8, 2);
        let _ = hola.add(9, 2);
        let _ = hola.add(1, 0);
        let _ = hola.add(2, 1);
        println!("{:#?}", hola);
        //let _ = hola.add(2, 1);
        println!("{:#?}", hola);
        println!("{:#?}", hola);
        /*
        hola.add(101, 1);
        hola.add(102, 1);
        hola.add(103, 1);
        hola.add(104, 1);
        */
        //println!("{:#?}",hola);
        /*
        println!("\n\n\n\n");
        hola.add(55, 0);
        println!("{:#?}",hola);
        */
        //hola.add(125, 1);
        //hola.add(25, 1);
        //println!("{:#?}",hola);
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
