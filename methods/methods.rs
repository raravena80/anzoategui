#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }

    fn diagonal(&self) -> u32 {
       (((self.width * self.width) + (self.height * self.height)) as f64).sqrt() as u32
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The perimeter of the rectangle is {} pixels.",
        rect1.perimeter()
    );

    println!(
        "The diagonal of the rectangle is {}",
        rect1.diagonal()
    );
}
