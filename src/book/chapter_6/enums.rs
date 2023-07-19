pub fn main() {
    let ip_v4 = IpAddr::V4;
    let ip_v6 = IpAddr::V6;

    println!("ip_v4: {:?}", ip_v4);
    println!("ip_v6: {:?}", ip_v6);
}

#[derive(Debug)]
enum IpAddr {
    V4,
    V6
}

