// Reference Pointers - point to a resource in memory

pub fn run() {
    // Primitive array 
    let arr1 = [1, 2, 3];
    // Deep copy - entire different stack and heap allocation
    let arr2 = arr1;

    // With non-primitives, if you assign another variable to a piece of dat, the first
    // variable will no longer hold that valu. You'll need to use a reference (&) to point to 
    // the resource. 

    let vec1: Vec<i32> = vec![1, 2, 3];
    // Make a non-mutable reference to the first vector
    let vec2: &Vec<i32> = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}