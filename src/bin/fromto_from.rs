use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug)]
struct Float {
    value: f32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<f32> for Float {
    fn from(item: f32) -> Self {
        Float { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let float = Float::from(50.0);
    println!("My float is {:?}", float);
}
