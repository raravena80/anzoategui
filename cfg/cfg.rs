// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(target_os = "macos")]
fn are_you_on_mac() {
    println!("You are running mac!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(not(target_os = "macos"))]
fn are_you_on_mac() {
    println!("You are *not* running mac!");
}

fn main() {
    are_you_on_linux();
    are_you_on_mac();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else if cfg!(target_os = "macos") {
        println!("Yes. It's definitely mac!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
