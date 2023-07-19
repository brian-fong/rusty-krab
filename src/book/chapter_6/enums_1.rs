pub fn main() {
    println!();
    println!("+-----------------+");
    println!("|   Enums pt. 1   |");
    println!("+-----------------+");

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

