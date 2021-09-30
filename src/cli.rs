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
    }

}
