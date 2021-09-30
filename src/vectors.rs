// Vectors - resizeable arrays

use std::mem;

pub fn run() {

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // Reassign a value
    numbers[2] = 10;

    // Add on to vector
    numbers.push(6);
    numbers.push(122);

    // Pop off last value:
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vector are heap allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    // reference to the vector of numbers
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);

}