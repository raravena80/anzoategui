

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let wordlength = first_word(&s);
    println!("{}", wordlength);
    s.clear();

    // Another slice
    let mut s2 = String::from("Not sure what this is");
    let wl = first_word(&s2);
    println!("{}", wl);
    s2.clear();
}
