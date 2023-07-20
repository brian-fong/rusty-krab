use std::collections::HashMap;

pub fn main() {
    println!();
    println!("+---------------+");
    println!("|   Hash Maps   |");
    println!("+---------------+");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("scores: {:?}", scores);

    let team_name = String::from("Yellow");
    scores.insert(team_name, 20);
    let score = scores.get("Purple").copied().unwrap_or(0);
    println!("Purple: {} points", score);

    for (key, value) in scores {
        println!("{}: {} points", key, value);
    }

    println!();
}
