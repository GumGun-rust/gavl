pub mod drop;
pub mod clone;

use super::*;

pub fn test() {
    println!("hola");
    drop::log_drop();
    super::log();
}

pub fn lol() {
    println!("test");
}
