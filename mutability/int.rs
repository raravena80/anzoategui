
use std::sync::Arc;


fn main() {

    let x = Arc::new(5);
    let y = x.clone();

    println!("{}", x);
    println!("{}", y);

    let a = Arc::new("hi");
    let b = a.clone();

    println!("{}", a);
    println!("{}", b);
}
