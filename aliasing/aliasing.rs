// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
type Inch = u64;
type Foot = u32;
type Meter = u32;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute

#[allow(non_camel_case_types)]
type u32_t = u32;

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    let feet: Foot = 2 as u32_t; 
    let meters: Meter = 3 as u32_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    println!("{} feet", feet);
    println!("{} meters + {} feet = {} unit?",
             meters,
             feet,
             meters + feet);
}
