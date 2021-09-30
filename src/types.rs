// Primative types
// Integers: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 (number of bits)
// Floats: f32, f64
// Booleans (bool)
// Characters (char)
// Tuples
// Arrays

pub fn run() {
	// Default is "i32"
	let x = 1;

	// Default is "f64"
	let y = 2.5;

	// Add explicit type
	let z: i64 = 1000000000;

	// Find max size
	println!("Max i32: {}", std::i32::MAX);
	println!("Max i64: {}", std::i64::MAX);

	// Boolean
	let is_active: bool = true;

	// Get boolean from expression
	let is_greater: bool = 10 > 11;

	let a1: char = 'a';
	let face: char = '\u{263a}';
	
	// explicit type casting 
	let _num: f32 = 10f32;
	let _num2: f32 = as f32;

	println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}