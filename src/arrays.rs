// Array - fixed list where elements are of the same type

use std::mem;

pub fn run() {
    // [type; length], has to have a number of elements equal to the size
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Reassign a value
    numbers[2] = 10;

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    // reference to the array of numbers
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

}