fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else if n % 1 == 0 {
            println!("buzzfizz");
        } else {
            println!("{}", n);
        }
    }

    for i in 102..302 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 1 == 0 {
            println!("buzzfizz");
        } else {
            println!("{}", i);
        }
    }
}
