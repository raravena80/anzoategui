#![allow(unreachable_code)]

fn main() {
    'mostouter: loop {
        println!("Entered the most outer loop");
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // This would break only the inner loop
                //break;

                // This breaks the outer loop
                break 'outer;
            }

            println!("This point will never be reached");
        }
        println!("Exited the outer loop");
        break 'mostouter;
    }
    println!("Exited the most outer loop");
}
