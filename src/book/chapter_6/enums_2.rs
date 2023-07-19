#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called");
    }
}

pub fn main() {
    println!();
    println!("+-----------------+");
    println!("|   Enums pt. 2   |");
    println!("+-----------------+");
    let m = Message::Write(String::from("hello"));
    println!("m: {:?}", m);
    m.call();

    println!();
    println!("+-------------+");
    println!("|   Options   |");
    println!("+-------------+");
    let some_number = Some(5);
    let some_char = Some("f");
    let absent_number: Option<i32> = None;
    // Note: absent_number needs type annotation since the compiler 
    // cannot infer the type from just None.
    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);

    // Note: The following produces an error since we're trying to add 
    // two values of different types (i8 and Option<i8>)
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    println!();
}

// Definition in the standard library:
// enum Option<T> {
//     None,
//     Some(T),
// }

