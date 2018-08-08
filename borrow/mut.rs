
fn main() {
	let mut x = 5;
	{
    	let y = &mut x;
    	*y += 1;
	}
	println!("{}", x);

    let mut z = 10;
    {
        let zz = &mut z;
        *zz +=2;
    }
	println!("{}", z);

}
