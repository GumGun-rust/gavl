mod avl;


fn main() {
    println!("Hello, world!");
    avl::log();
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test(){
        //avl::log();
        let mut hola = avl::Map::<u64,u64>::new();
        //println!("{:#?}",hola);
        hola.add(100, 1);
        hola.add(50, 0);
        hola.add(25, 0);
        println!("\n\n\n\n");
        hola.add(55, 0);
        println!("{:#?}",hola);
        /*
        */
        //hola.add(125, 1);
        //hola.add(25, 1);
        //println!("{:#?}",hola);
    }
}
