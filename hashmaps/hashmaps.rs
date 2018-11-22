use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() { 
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Gabriel", "645-7689");
    contacts.insert("Camilla", "435-8291");
    contacts.insert("MariaInes", "555-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns true 
    // if the inserted value is new, false otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Camilla") {
        Some(&number) => println!("Calling Camilla: {}", call(number)),
        _ => println!("Don't have Camilla's number."),
    }

    contacts.remove(&("Camilla")); 

    // `HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }
}
