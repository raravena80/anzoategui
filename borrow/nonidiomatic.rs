
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}


fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 3, 3];

    let (v1, v2, answer) = foo(v1, v2);
    println!("{:?}, {:?}, {}", v1, v2, answer);
}
