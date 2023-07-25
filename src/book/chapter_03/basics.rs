pub fn main() {
    tuples_and_arrays();

    print_pair(10, "hello");

    loops();
}

pub fn tuples_and_arrays() {
    // Tuples
    let tup: (i32, f32, &str) = (100, 3.14, "hello");
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    // Arrays
    let months: [&str; 12] = [
        "Jan", "Feb", "Mar",
        "Apr", "May", "Jun",
        "Jul", "Aug", "Sep",
        "Oct", "Nov", "Dec"
    ];
    println!("Array: {}", months[9]);

    println!();
}

pub fn print_pair(x: i32, y: &str) {
    println!("Pair: ({x}, {y})");

    println!();
}

pub fn loops() {
    let mut timer: u32 = 3;

    println!("Counting down...");

    // Using a while-loop
    while timer > 0 {
        println!("{}!", timer);
        timer = timer - 1;
    }

    // Alternatively, using a for-loop and a number range
    // for number in (1..4).rev() {
    //     println!("{}", number);
    // }

    println!("LET'S GOOOOO!!!!!\n");

    let arr = ["hello", "there", "my", "friend!"];
    
    for element in arr.iter() {
        println!("{}", element);
    }

    println!();
}

