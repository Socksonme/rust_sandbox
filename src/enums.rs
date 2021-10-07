// Enums are types which have a few definitive values

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColour(u8, u8, u8)
}

impl Message {
    fn some_function() {
        println!("Random message");
    }
}

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

// If you pass by type "Movement", you take ownership of the variable using the parameter "m",
// however, if you pass by tye "&Movement", you pass by reference and borrow the value, instead of copying and moving it
// allowing you to pass the same variable again
// Note: not a &mut reference; (Either you can have many non-mutable references, or one mutable one)
fn move_avatar (m: &Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
}

pub fn run() {

    let four: IpAddrKind = IpAddrKind::V4(127, 122, 10, 2);
    let six: IpAddrKind = IpAddrKind::V6("aaa".to_string());

    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);

    let movement1: Movement = Movement::Up;
    let movement2: Movement = Movement::Down;
    let movement3: Movement = Movement::Left;
    let movement4: Movement = Movement::Right;

    move_avatar(&movement1);
    move_avatar(&movement2);
    move_avatar(&movement3);
    move_avatar(&movement4);
}