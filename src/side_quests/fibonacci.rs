use std::io::{self, Write};

pub fn main() {
    loop {
        let mut n: String = String::new();

        println!("\nFibonnaci:");

        print!("n = ");
        io::stdout().flush()
            .expect("Failed to flush output!");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line!");

        let n: u32 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("n must be a non-negative integer!\n");
                continue;
            },
        };

        let fib: u64 = get_nth_fib(n);
        println!("The {n}-th Fibonacci number is: {fib}");
        break;
    }
}

pub fn get_nth_fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return get_nth_fib(n-2) + get_nth_fib(n-1);
    }
}

