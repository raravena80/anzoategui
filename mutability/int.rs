
use std::sync::Arc;


fn main() {

    let x = Arc::new(5);
    let y = x.clone();

    println!("{}", x);
    println!("{}", y);
             
}
