pub fn run() {
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
        
        // unwrap returns the Ok value, or it panics, provides the Err(value) value, however, this should only be used
        // when the program cannot continue working after an error.
        // It is also reccomended to handle the error yourself, or use .expect(), instead of .unwrap()
        let num3: i32 = test2(11).unwrap();
        println!("{}", num3);
}

// first Result type is Ok, second is Err
fn test(int: i32) -> Result<i32, String> {
    match int {
        // Match an inclusive range from 0 to 100 and return the number
        0 ..= 100 => Ok(int),
        // Returns an error with a string explaining it
        _ => Err(String::from("Invalid number"))
    }
}
// Options should be used when you are expecting nothing are something,
// such as some kind of a search/find command 
fn test2(int: i32) -> Result<i32, String> {
    match int {
        // Match guard
        // x is binded to the value and then is checked if it matches the match guard
        x if x > 0 && x <= 100 => Ok(x * 2), // if x > 0 && x <= 100 - match guard

        // You can also bind a pattern to a name with '@', so it'd be something like:
        // x @ 0..=100 => return Ok(x * 2),

        _ => Err("Invalid number".to_string())
    }
}