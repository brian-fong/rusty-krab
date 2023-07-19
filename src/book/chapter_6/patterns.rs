use crate::book::utils::us_states::UsState;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This coin is from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

pub fn main() {
    println!();
    println!("+-----------+");
    println!("|   Coins   |");
    println!("+-----------+");

    // let coin: Coin = Coin::Penny;
    let coin: Coin = Coin::Quarter(UsState::California);
    println!("coin: {:?}", coin);

    let value: u8 = value_in_cents(&coin);
    println!("value: {}", value);

    println!();

    println!();
    println!("+-----------------------------+");
    println!("|   Matching with Option<T>   |");
    println!("+-----------------------------+");

    let five = Some(5);
    let six = plus_one(five);
    let none = None;
    let none_plus = plus_one(none);

    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
    println!("none_plus: {:?}", none_plus);

    println!();
    println!("+-----------------------+");
    println!("|   if-let Statements   |");
    println!("+-----------------------+");
    let config_max = Some(3u8);
    // The following match statement prints if the value is Some and does
    // nothing otherwise 
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // The following if-let statement is equivalent to the match above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    println!();
}

