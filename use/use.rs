#![allow(dead_code)]

enum Status {
    Rich,
    Middle,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
    Redcross,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Middle, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Middle => println!("The middle class has some money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
        Redcross => println!("Red Cross members help!"),
    }
}
