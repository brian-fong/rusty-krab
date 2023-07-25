use std::{ io, io::Write, cmp::Ordering };
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
   pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100; Got {}", value);
        }
        return Guess { value };
    } 

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

pub fn main() {
    println!();
    println!("+-------------------+");
    println!("|   Guessing Game   |");
    println!("+-------------------+");
    println!("Guess a random number from 1 to 100!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Guess: ");
        io::stdout().flush()
            .expect("Failed to flush output!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let value: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            },
        };

        let guess = Guess { value };

        if guess.value() < 1 || guess.value() > 100 {
            println!("The secret number is between 1 and 100");
            continue;
        }

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }

        println!();
    }
}
