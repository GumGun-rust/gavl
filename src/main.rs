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
        hola.add(12, 12);
    }
}
