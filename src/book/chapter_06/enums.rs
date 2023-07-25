pub fn main() {
    println!();
    println!("+----------------------+");
    println!("|   Enum: IP Address   |");
    println!("+----------------------+");

    // let ip_v4 = IpAddrKind::V4;
    // let ip_v6 = IpAddrKind::V6;
    //
    // let ip_home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let ip_loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let ip_home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let ip_loopback = IpAddrKind::V6(String::from("::1"));

    let ip_home = IpAddr::V4(127, 0, 0, 1);
    let ip_loopback = IpAddr::V6(String::from("::1"));

    // println!("ip_v4: {:?}", ip_v4);
    // println!("ip_v6: {:?}", ip_v6);

    println!("ip_home: {:?}", ip_home);
    println!("ip_loopback: {:?}", ip_loopback);

    println!();
    println!("+-------------------+");
    println!("|   Enum: Message   |");
    println!("+-------------------+");
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

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

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

