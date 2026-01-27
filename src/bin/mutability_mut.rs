

fn main() {


    let _x = 5;
    //x = 6;
    let mut y = 8;
    println!("{}", y);
    y = 9;
    println!("{}", y);


    let mut z = 5;
    let h = &mut z;
    // This would fail since h is inmutable
    //h = &mut y;
    // But this won't
    *h = 8;
    println!("{}", *h);


    let mut a = 20;
    let mut b = &mut a;
    println!("{}", b);
    // This now works because b is mut
    b = &mut y;
    println!("{}", b);
}
