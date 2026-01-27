struct Cardinal;
struct BlueJay;
struct Turpial;
struct Tucan;
struct Turkey;

trait Red {}
trait Blue {}
trait Yellow {}
trait MultiColor {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
impl Yellow for Turpial {}
impl MultiColor for Tucan {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
fn yellow<T: Yellow>(_: &T) -> &'static str { "yellow" }
fn multicolor<T: MultiColor>(_: &T) -> &'static str { "multicolor" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let turpial = Turpial;
    let tucan = Tucan;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    println!("A turpial is {}", yellow(&turpial));
    println!("A tucan is {}", multicolor(&tucan));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
