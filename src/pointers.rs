pub fn run() {
	// Primitives
	let mut a = [1,2,3];
	let mut b = a; // Copied a or points to a?
	println!("a b: {:?}", (a, b));
	b[0] = 12;
	println!("a b: {:?}", (a, b));

	// Vectors
	let mut c = vec![1,2,3];
	let mut d = &c; // = will move a vec without using &
	println!("c d: {:?}", (&c, d));
	//c.push(12); cant because something is borrowing it as immutable
	//d.push(12); can't because reference is immutable

}
