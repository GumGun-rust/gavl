use super::*;


pub(crate) fn print_type_of<KeyType>(_: &KeyType) {
    println!("{}", std::any::type_name::<KeyType>())
}

mod avl_test{
    use super::*;

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

    /*
    #[ignore]
    #[test]
    fn test_iter(){
        let mut avl = Map::<u64,u64>::new();
        for number in 0..7 {
            avl.add(number, 0).unwrap();
        }
        println!("{:#?}", avl);
        let iter_level = avl.level_iter();//.enumerate();
        
        for elem in iter_level {
            println!("{:?}", elem);
        }
        //println!("{:#?}", avl);
        //panic!();
        //let hola = 
    }
    */
}
