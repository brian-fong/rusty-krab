use std::{ io, io::Write, cmp::Ordering };
use rand::Rng;

pub fn main() {
    println!("Guess a random number from 1 to 100!");

    let sol: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Guess: ");
        io::stdout().flush()
            .expect("Failed to flush output!");

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

        println!("\n");
    }
}

