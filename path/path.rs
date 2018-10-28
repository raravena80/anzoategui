use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show`able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    let new_path = path.join("a").join("b").join("c");

    // Convert the path into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    let path2 = Path::new("m");
    let _display2 = path2.display();

    let new_path2 = path.join("n").join("o").join("q");

    match new_path2.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    let path3 = Path::new("uu");
    let _display3 = path3.display();

    let new_path3 = path.join("jj").join("oo").join("aa");

    match new_path3.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

}
