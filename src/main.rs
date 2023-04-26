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
        avl::log();
        let mut hola = avl::Map::<u64,u64>::new();
        println!("{:#?}",hola);
        hola.add(12, 12);
        println!("{:#?}",hola);
    }
}
