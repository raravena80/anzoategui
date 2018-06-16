use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("4").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
