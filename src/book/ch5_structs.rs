pub fn main() {
    let user1 = User {
        active: true,
        name: String::from("Brian"),
        username: String::from("0xfrian"),
        sign_in_count: 1,
    };

    let user2 = User {
        name: String::from("Jonathan"),
        username: String::from("antispam"),
        ..user1
    };

    println!("\nuser1: {:?}", user1);
    println!("user2: {:?}\n", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
}

#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    username: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn build_user(name: String, username: String) -> User {
    return User {
        active: true,
        name,
        username,
        sign_in_count: 1,
    };
}

