// #[cfg(test)]
// pub mod tests {
//     use super::greet;
//
//     #[test]
//     fn test_greet() {
//         let name = String::from("frian");
//         let greeting = greet(&name);
//         assert!(
//             greeting.contains(&name),
//             "Greeting did not contain '{}', value was '{}'",
//             name,
//             greeting,
//         );
//     }
//
//     use super::Rectangle;
//
//     #[test]
//     fn test_rect_pass() {
//         let rect_a = Rectangle { width: 10, height: 15 };
//         let rect_b = Rectangle { width: 15, height: 30 };
//         assert!(rect_b.contains(&rect_a));
//     }
//
//     #[test]
//     fn test_rect_fail() {
//         let rect_a = Rectangle { width: 10, height: 15 };
//         let rect_b = Rectangle { width: 15, height: 30 };
//         assert!(!rect_a.contains(&rect_b));
//     }
// }

pub fn greet(name: &str) -> String {
    return format!("hello {}", name);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn contains(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub fn main() {
    println!();
    println!("+-----------+");
    println!("|   Tests   |");
    println!("+-----------+");
}
