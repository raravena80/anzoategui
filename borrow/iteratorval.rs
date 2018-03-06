
fn main() {
    let v = vec![1, 2, 3];
    let mut z: Vec<i32> = Vec::new();
    for i in &v {
        println!("{}", i);
        z.push(34);
        // The following would fail
        // v cannot last longer than the borrowed &v
        v.push(34);
    }
    println!("{}", z[0]);
}
