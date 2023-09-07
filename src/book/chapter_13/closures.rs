use std::thread;

// === Rectangle ===
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// === Inventory ===
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // If user_preference is defined, then return the value
        // else return the value from the given closure function
        let result = user_preference.unwrap_or_else(|| {
            return self.most_stocked();
        });
        return result;
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}


pub fn main() {
    println!();
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    // let user_pref = Some(ShirtColor::Red);
    let user_pref = None;
    let giveaway = store.giveaway(user_pref);
    println!(
        "The user with preference {:?} gets a {:?} shirt",
        user_pref,
        giveaway
    );
    println!();

    // === Borrowing with closures ===
    // Example #1: Borrowing immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("Before calling closure: {:?}", list);
    println!();

    // Example #2: Borrowing mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(4);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
    println!();

    // === Passing closure to new thread ===
    // Example #1
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    println!();

    // === Fn traits in closures ===
    let mut list =[
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    println!("Rectangles (unsorted): {:?}", list);

    list.sort_by_key(|r| r.width);
    println!("Rectangles (sorted): {:?}", list);
    println!();
}
