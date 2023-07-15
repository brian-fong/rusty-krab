use std::io::{self, Write};

pub fn main() {
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

pub fn convert(temp: f64) -> f64 {
    let temp_c: f64 = (temp - 32.0) * (5.0/9.0);
    return temp_c;
}

