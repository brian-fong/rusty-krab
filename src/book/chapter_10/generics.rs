#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }

    fn y(&self) -> &T {
        return &self.y;
    }
}

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

pub fn main() {
    println!();
    println!("+--------------+");
    println!("|   Generics   |");
    println!("+--------------+");
    let integer_point = Point { x: 5, y: 10 };
    println!("integer_point: {:?}", integer_point);
    let float_point = Point { x: 2.0, y: 5.5 };
    println!("float_point: {:?}", float_point);
    // let mixed_point = Point { x: 2.0, y: 10 };
    // println!("mixed_point: {:?}", mixed_point);
    println!();

    let point = Point { x: 3, y: 6 };
    println!("point (x, y): ({}, {})", point.x(), point.y());

    println!();
}
