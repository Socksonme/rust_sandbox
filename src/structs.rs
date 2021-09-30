// Structs - used to created custom data types

// Traditional struct
/* struct Color {
    red: u8,
    green: u8,
    blue: u8
} */

// Tuple struct
/* struct Color(u8, u8, u8); */

// Structs have an extra level of visibility with their fields. 
// The visibility defaults to private, and can be overridden with the pub modifier. 
// This visibility only matters when a struct is accessed from outside the module where it is defined,
// and has the goal of hiding information (encapsulation).
pub struct Person {
    pub first_name: String,
    pub last_name: String
}

impl Person {
    // Construct the person
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        };
    }

    // self acts like "this" 
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        return (self.first_name, self.last_name);
    }
}

pub fn run() {
    /* let mut c: Color = Color{red: 255, green: 0, blue: 0};
    
    c.red = 200;

    println!("{} {} {}", c.red, c.green, c.blue); */

    /* let mut c: Color = Color(255, 0, 0);

    c.0 = 200;

    println!("{} {} {}", c.0, c.1, c.2); */

    let mut p: Person = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person full name: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person full name: {}", p.full_name());
    println!("Person tuple name: {:?}", p.to_tuple());
}