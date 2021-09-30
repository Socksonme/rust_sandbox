// Loops are used to iterate until a condition is met

pub fn run() {
    let mut count = 0;
    let mut fizz_string: String = String::new();

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop (FizzBuzz)

    while count < 100 {
        if count % 3 == 0 {
            fizz_string.push_str("Fizz");
        }
        if count % 5 == 0 {
            fizz_string.push_str("Buzz");
        }

        if fizz_string.is_empty() {
            fizz_string = format!("{}", count);
        }

        println!("{}", fizz_string);
        count += 1;
        fizz_string.clear();
    }

    // For Range loop (x..y excludes y, ..= would be inclusive)
    for x in 0..100 {
        if x % 3 == 0 {
            fizz_string.push_str("Fizz");
        }
        if x % 5 == 0 {
            fizz_string.push_str("Buzz");
        }

        if fizz_string.is_empty() {
            fizz_string = format!("{}", x);
        }

        println!("{}", fizz_string);
        fizz_string.clear();
    }
}