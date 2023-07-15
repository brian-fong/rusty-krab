use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("Guess a random number from 1 to 100!");

    let sol: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            },
        };

        match guess.cmp(&sol) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!();
    }
}

