
fn take(v: Vec<i32>) {
    // This function has taken onership of v
    println!("v[0] is {}", v[0]);
}

fn main() {

    let v = vec![1, 2, 3];
    take(v);
    // The following would fail
    //println!("v[0] is {}", v[0]);
}
