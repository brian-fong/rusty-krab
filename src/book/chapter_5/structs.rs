pub fn main() {
    println!();
    println!("+-------------+");
    println!("|   Structs   |");
    println!("+-------------+");

   // Examples of structs
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

    println!("user1: {:?}", user1);
    println!("user2: {:?}", user2);

    println!();
    println!("+-------------------+");
    println!("|   Tuple Structs   |");
    println!("+-------------------+");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);

    println!();
    println!("+-------------------------------------------------+");
    println!("|   More Structs; Methods; Associated Functions   |");
    println!("+-------------------------------------------------+");
    let width = 30;
    let height = 50;
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(width * scale),
        height,
    };

    let rect_1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("{:#?}", rect);
    dbg!(&rect);
    println!("The area of the rectangle is {} square pixels",
        // calculate_area(width, height)
        // calculate_area(&rect)
        rect.area()
    );

    println!("rect_1: {:#?}", rect_1);
    println!("rect_2: {:#?}", rect_2);
    println!("rect_1 contains rect_2: {}", rect_1.contains(&rect_2));

    // Example of associated functions
    let square = Rectangle::square(20);
    println!("square: {:#?}", square);

    println!();
}

// fn calculate_area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn calculate_area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }

fn calculate_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn contains(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
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

