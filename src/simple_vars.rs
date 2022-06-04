pub fn run() {
	//Indexed printing
	let dingo = "Karen";
	let two = "Yasss";
	let one = "Queens";
	println!("{0} has all the {2} and {1}", dingo, one, two);

	//Mutable variables
	let mut status = "alive";
	println!("Count choco is {}", status);
	status = "dead";
	println!("Count choco is now {}", status);

	//Constants need to be defined
	const GUID: i32 = 501;
	println!(
		"Count choco identifies as a member of {} even though he died a line ago, how is he still polling?", 
		GUID
	);

	// Why do this?
	let (name, weight, age) = ("Countess Choco", "REDACTED", "How dare you!");
	println!(
		"Our next guest is {} and her age is {}, with a weight of {}.", 
		name, 
		age, 
		weight
	);

	//Unspecified number of vars
	println!("The world is a: {:?}", (status, dingo, one, name, two, weight));

	//Var types
	/*
	Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
	Floats: f32, f64
	Bools: bool (NO TRUTHY BUSINESS IN RUST! true|| false)
	Character: char
	Tuple
	Array: [(datatype);(numof)] let mut array: [i32; 7] = [0; 7];
	*/

	//Characters - use single quotes
	let c_force = 'U';
	let dirty_bird = '\u{1F601}';
	println!("In the wilds the {} bird can been heard shouting {}", c_force, dirty_bird);

	//Strings
	/*
	str: Immutable
	String: Mutable (if set to mut), heap?
	*/
	let mut velvet = String::from("Saucy");
	println!("{} is baucy", velvet);
	velvet = velvet + " oooh";
	println!("{}: how is this different from a regular str? Because it IS", velvet);
	velvet.push(' ');
	println!("{}", velvet);
	velvet.push_str("Dirty Gurl!");
	println!("{}", velvet);
	println!("Length: {}", velvet.len());
	println!("Emptay: {}", velvet.is_empty());
	//replace returns a string without modifying the original
	let new_velvo = velvet.replace("Gurl", "Fool");
	println!("Velvet Contains 'Gurl'? {}", velvet.contains("Gurl"));
	println!("Velvo Contains 'Gurl'? {}", new_velvo.contains("Gurl"));
	//loop
	let looper = String::from("Don't you dare loop me, I'll come for you!");
	for word in looper.split_whitespace() {
		println!("{}", word);
	}
	//assert
	let mut limits = String::with_capacity(5);
	limits.push_str("123456789");
	println!("Capacity {}", limits.capacity());
	println!("{}", limits);
	assert_eq!(10, limits.capacity());

	//Tuples
	/*
		Max is 12 elements

	*/
	let car: (&str, &str, i8) = ("Honda", "Shivic", 55);
	println!("I drive a {} {} and it goes {} Clickometers/day", car.0, car.1, car.2);

	//Arrays
	let num1: [i32; 5] = [1, 2, 3, 4, 5];
	let num2: [i32; 5] = [1;5];
	println!("Num1: {:#?}", num1); //Fancy print
	println!("Num2: {:?}", num2);
	println!("Size: {} bytes", std::mem::size_of_val(&num2)); //Can also add using std::mem at top of file similar to cpp
	let kimbo: &[i32] = &num1[0..3];
	println!("Slice: {:?}", kimbo);

	//Vectors
	let mut vee: Vec<i32> = vec![1, 2, 3, 4, 5];
	println!("Vee: {:?}", vee);
	println!("Vee Size: {} bytes", std::mem::size_of_val(&vee));
	vee.push(17);
	println!("Vee: {:?}", vee);
	for v in vee.iter() {
		println!("{}", v);
	}
	for v in vee.iter_mut() {
		*v += 11;
	}
	println!("Vee: {:?}", vee);
}
