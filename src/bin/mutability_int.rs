
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


    let h = Arc::new(1.5);
    let i = h.clone();

    println!("{}", h);
    println!("{}", i);
}
