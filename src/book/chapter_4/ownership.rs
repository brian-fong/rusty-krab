pub fn main() {
    // // Example of moving ownership
    // let s = String::from("hello");
    // println!("\ns = {s}");
    // take_ownership(s);
    // println!("s = {s}\n");      // s is invalid here
    //
    // // Example of copying ownership
    // let i: i32 = 5;
    // println!("\ni = {i}");
    // make_copy(i);
    // println!("i = {i}\n");      // i is still valid here

    // // Example of taking away and returning ownership
    // let s1 = String::from("hello");
    // println!("s1: {s1}");
    // let s2 = borrow_ownership(s1);
    // println!("s1: {s1}");
    // println!("s2: {s2}");
    // // NOTE: s1 is "moved", making it invalid. Furthermore, s2 is now
    // // the new owner. This means at the end of the scope, only s2 will
    // // be dropped and nothing happens to s1

    // // Example of using references (borrowing ownership)
    // let s = String::from("hello");
    // let length = get_length(&s);
    // println!("The length of {s} is {length}");

    // Example of using mutable references
    let mut s = String::from("hello");
    println!("s = {s}");
    add_word(&mut s);
    println!("s = {s}");
    
    // // Example of dangling pointers
    // let p = dangling_pointer();
}

pub fn take_ownership(s: String) {
    println!("Taking ownership of s: {s}...");
}

pub fn make_copy(i: i32) {
    println!("Making copy of i: {i}...");
}

pub fn borrow_ownership(s: String) -> String {
    return s;
}

pub fn get_length(s: &String) -> usize {
    return s.len();
}

pub fn add_word(s: &mut String) {
    s.push_str(" world!");
}

// pub fn dangling_pointer() -> &String {
//     let s = String::from("hello");
//     return &s;
// }

