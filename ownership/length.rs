

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}


fn main() {

    let s1 = String::from("hello");
   
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len); 


    let k1 = String::from("Another string");
    
    let (k2, len2) = calculate_length(k1);
    println!("The length of '{}' is {}.", k2, len2);
}
