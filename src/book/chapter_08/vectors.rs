pub fn main() {
    println!();
    println!("+----------------------+");
    println!("|   Creating Vectors   |");
    println!("+----------------------+");
    let vec_1: Vec<i32> = Vec::new();
    println!("vec_1: {:?}", vec_1);

    let vec_2 = vec![1, 2, 3];
    println!("vec_2: {:?}", vec_2);

    let mut vec_3 = Vec::new();
    vec_3.push(4);
    vec_3.push(5);
    vec_3.push(6);
    println!("vec_3: {:?}", vec_3);

    println!();
    println!("+----------------------+");
    println!("|   Indexing Vectors   |");
    println!("+----------------------+");
    let vec_4: Vec<i32> = vec![10, 20, 30];
    println!("vec_4: {:?}", vec_4);

    let element = vec_4[1];
    println!("element using index: {:?}", element);

    // let element = vec_4.get(2);
    let element = vec_4.get(4);  // outside range
    match element {
        Some(element) => println!("element using get(): {:?}", element),
        None => println!("no element found"),
    }

    // The following bit of code showcases an example of creating an 
    // immutable reference to a vector element and a mutable reference 
    // to mutate the vector. Referencing the element after the 
    // mutation results in the compiler panicking because the element 
    // is dependent on the whole vector; thus we cannot have both 
    // mutable and immutable references to the vector's contents and 
    // the vector itself.
    // let element = &vec_4[0];
    // vec_4.push(40);
    // println!("element: {:?}", element);

    println!();
    println!("+--------------------------+");
    println!("|   Looping Over Vectors   |");
    println!("+--------------------------+");
    let mut vec_5: Vec<i32> = vec![100, 32, 57];
    println!("vec_5: {:?}", vec_5);

    println!("Immutable references: ");
    for i in &vec_5 {
        println!("  - {i}");
    }

    println!("Mutable references: ");
    for i in &mut vec_5 {
        *i *= 2;
        println!("  - {i}");
    }

    println!();
    println!("+----------------------+");
    println!("|   Enums in Vectors   |");
    println!("+----------------------+");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(5.5),
    ];
    println!("Row: {:?}", row);

    println!();
}

