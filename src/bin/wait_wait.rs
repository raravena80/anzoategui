use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("4").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of first batch");

    let mut child2 = Command::new("sleep").arg("4").spawn().unwrap();
    let _result2 = child2.wait().unwrap();
    println!("reached end of main");
}
