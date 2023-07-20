pub fn main() {
    println!();
    println!("+----------------------+");
    println!("|   Creating Strings   |");
    println!("+----------------------+");

    let msg = "hello";
    println!("msg: {}", msg);

    let s_1 = msg.to_string();
    println!("s_1: {}", s_1);

    let s_2 = String::from("hello");
    println!("s_2: {}", s_2);

    println!();
    println!("+-------------------+");
    println!("|   Concatenation   |");
    println!("+-------------------+");
    let mut s_3 = String::from("hola");
    s_3.push_str(" mundo!");
    println!("s_3: {}", s_3);

    let hello = String::from("hello");
    let space = String::from(" ");
    let world = String::from("world!");
    let sentence_1: String = hello + &space + &world;
    println!("sentence_1: {}", sentence_1);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let sentence_2: String = format!("{tic}-{tac}-{toe}");
    println!("sentence_2: {}", sentence_2);

    println!();
    println!("+---------------------+");
    println!("|   Slicing Strings   |");
    println!("+---------------------+");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("hello: {}", hello);
    println!("s: {}", s);

    println!();
    println!("+----------------------------+");
    println!("|   Iterating Over Strings   |");
    println!("+----------------------------+");
    let msg = "asdf";
    println!("msg: {}", msg);
    for char in msg.chars() {
        println!("char: {}", char);
    }
    for byte in msg.bytes() {
        println!("byte: {}", byte);
    }

    println!();
}

