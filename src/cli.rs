use std::env;

// Basic CLI
pub fn run() {
    // args passed in the command line
    let args: Vec<String> = env::args().collect();
    let name: String = "Brad".to_string();
    let status: String = "100%".to_string();
    if args.capacity() > 1 {
        // args[0] is path
        let command: String = args[1].clone();

        // Turn the String into &str, as String dereferences to &str (a string slice)
        match command.as_str() {

            // The left hand side of the match arm is the pattern, and it cannot include functions
            // - that's why you have to turn the String into &str
            "hello" => println!("Hi {}, how are you?", name),
            "status" => println!("Status: {}", status),
            // "_" acts as a default case
            _ => println!("That is not a valid command.")
        }

        // When you write unwrap() in your program, you are saying:
        // At this point, a None/Err(_) value is a programming error and the program is unable to recover from it.
        let res: Result<i32, String> = test(110);
        
        // "nearly every statement is also an expression" - some guy on SO; assigning variable through match statement
        let num: i32 = match res {
            // Result::Ok, containg some type T value
            Ok(value) => value,

            // Handle the error, without using the value assigned to Result::Err
            Err(_) => {
                // Sequence of statements
                println!("Error: Number is not between 0 and 100.");
                -1
            }
        };
        
        // returns the Ok value, or it panics, provides an error messages and the contents of the Err
        let num2: i32 = test(90).expect("Number not between 0 and 100.");
        
        println!("{} {}", num, num2);
        
        // unwrap returns the Some value, or it panics, without providing the error, as it's of type Option::None, instead of 
        // Result::Err, so it could be considred bad practice
        let num3: i32 = test2(11).unwrap();
        println!("{}", num3);
    }
}
// first Result type is Ok, second is Err
fn test(int: i32) -> Result<i32, String> {
    match int {
        // Match an inclusive range from 0 to 100 and return the number
        0 ..= 100 => return Ok(int),
        // Returns an error with a string explaining it
        _ => return Err(String::from("Invalid number"))
    }
}
// Option can either return Option::None or Option::Some(T)
fn test2(int: i32) -> Option<i32> {
    match int {
        // Match guard
        // x is binded to the value and then is checked if it matches the match guard
        x if x > 0 && x <= 100 => return Some(x * 2), // if x > 0 && x <= 100 - match guard

        // You can also bind a pattern to a name with '@', so it'd be something like:
        // x @ 0..=100 => return Some(x * 2),

        _ => return None
    }
}