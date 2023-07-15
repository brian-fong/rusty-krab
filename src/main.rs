// mod book;
mod side_quests;

// use crate::book::ch2_guess_game::main as ch2;
// use crate::book::ch3_basics::main as ch3;

use crate::side_quests::temperature::convert;

use std::io::{self, Write};

fn main() {
    loop {
        let mut temp_f: String = String::new();

        print!("\nTemperature in Fahrenheit: ");
        io::stdout().flush()
            .expect("Failed to flush output!");

        io::stdin()
            .read_line(&mut temp_f)
            .expect("Failed to read line");

        let temp_f: f64 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        println!("Input: {}", temp_f);

        let temp_c: f64 = convert(temp_f);

        println!("{}°F is {:.1}°C\n", temp_f, temp_c);

        break;
    }
}

