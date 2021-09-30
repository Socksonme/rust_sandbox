// Primative str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure

pub fn run() {
    let mut hello:String = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char prim
    hello.push('W');

    // Push string prim
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Check if contains
    println!("Contains \"World\": {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s:String = String::with_capacity(10);
    s.push('A');
    s.push('B');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}, {}", hello, s);
}