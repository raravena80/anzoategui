
fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn the_shortest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() { s1 }  else { s2 }
}

fn main() {
    let s1 = String::from("Apure");
    // explicitly borrowing to ensure that
    // the borrow lasts longer than s2 exists
    let s1_b = &s1;
    {
        let s2 = String::from("Guarico");
        let res = the_longest(s1_b, &s2);
        println!("{} is the longest if you judge by name", res);
    }

    let s3_b = &s1;
    {
        let s4 = String::from("Guarico");
        let res = the_shortest(s3_b, &s4);
        println!("{} is the shortest if you judge by name", res);
    }
}

