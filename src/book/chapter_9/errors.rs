use std::env;
use std::fs::{File, read_to_string};
use std::io::{ErrorKind, Read};
use std::path::{PathBuf, Path};

pub fn main() {
    // Manually panic by calling panic! macro
    // panic!("crash and burn!")

    // Enable backtracing
    env::set_var("RUST_BACKTRACE", "1");

    // Trigger "index out of bounds" error
    // println!();
    // println!("+-------------+");
    // println!("|   Vectors   |");
    // println!("+-------------+");
    // let v = vec![1, 2, 3];
    // println!("v[99] = {}", v[99]);

    // Build file path
    // let mut file_path: PathBuf = env::current_dir().unwrap();
    // file_path.push("src/book/utils/hello.txt");
    let file_path = Path::new("src/book/utils/hello.txt");
    println!("file_path: {:?}", file_path.display());


    println!();
    println!("+---------------------------+");
    println!("|   Error Handling: match   |");
    println!("+---------------------------+");
    println!("Opening file...");
    let file_result = File::open(&file_path);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            }
            _ => {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    };
    println!("File: {:?}", file);

    println!();
    println!("+------------------------------------+");
    println!("|   Error Handling: unwrap_or_else   |");
    println!("+------------------------------------+");
    println!("Opening file...");
    let file = File::open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File not found. Creating file...");
            File::create(&file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("File: {:?}", file);

    println!();
    println!("+----------------------------+");
    println!("|   Error Handling: expect   |");
    println!("+----------------------------+");
    println!("Opening file...");
    let file = File::open(&file_path)
        .expect("hello.txt should exist!");
    println!("File: {:?}", file);

    let file_content: String = read_to_string(&file_path).unwrap();
    println!("File Content: {:?}", file_content.trim());

    println!();
}
