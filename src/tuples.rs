// Tuples group together types
// Max 12 elements

pub fn run() {
    let person:(&str, &str, i8) = ("Denislav", "Stara Zagora", 27);
    
    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}