use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T,
) -> &'a str 
where 
    T: Display
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

#[derive(Debug)]
struct Quote<'a> {
    part: &'a str,
}

impl<'a> Quote<'a> {
    fn level (&self) -> i32 {
        return 22;
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("Attention please! {}", announcement);
        return self.part;
    }
}

fn first_word(s: &str) -> &str {
    // This function does not require explicit lifetime annotations.
    // Using Rule #2, since there is only 1 lifetime input parameter, 
    // that lifetime parameter is applied to all outputs.

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // - Implementation #1:
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }

    // - Implementation #2:
    // let result = String::from("the rusty krab pizza is the pizza for you and me");
    // return result.as_str();
}

pub fn main() {
    println!();
    println!("+---------------+");
    println!("|   Lifetimes   |");
    println!("+---------------+");

    // Examples of handling lifetimes
    // - Example #1
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // 
    // println!("r: {}", r);
    //
    // - Example #2
    // let x = 5;
    // let r = &x;
    // println!("r: {}", r);
    // println!();

    // Comparing the length of two strings
    // let a = String::from("abcd");
    // let b = "xyz";
    // let c = longest(a.as_str(), b);
    // if c == a {
    //     println!("'{}' is longer than '{}'", c, b);
    // } else if c == b {
    //     println!("'{}' is longer than '{}'", c, a);
    // }
    // println!();

    // More examples handling lifetimes
    // - Example #1
    // let a = String::from("arch linux is life");
    // {
    //     let b = String::from("ubuntu??");
    //     let c = longest(&a, &b);
    //     if c == a {
    //         println!("'{}' is longer than '{}'", c, b);
    //     } else if c == b {
    //         println!("'{}' is longer than '{}'", c, a);
    //     }
    // }
    // println!();
    // - Example #2
    // let a = String::from("arch linux is life");
    // let c;
    // {
    //     let b = String::from("ubuntu!!");
    //     c = longest(&a, &b);
    // }
    // println!("the longer string is {}", c);

    // Lifetime annotations with structs and methods
    // let intro = String::from("Hello. My name is Frian");
    // let first_sentence = intro.split(".").next()
    //     .expect("Could not find '.'");
    // let q = Quote {
    //     part: first_sentence,
    // };
    // let word = first_word(&intro);
    //
    // println!("intro: {}", intro);
    // println!("first sentence: {:?}", q.part);
    // println!("level: {:?}", q.level());
    // println!("announcement: {:?}", q.announce("Consider the following"));
    // println!("word: {:?}", word);
    // println!();

    let intro = String::from("Hello. My name is Frian");
    let a = String::from("abcd");
    let b = String::from("xyz");
    println!("longest: {}", longest_with_announcement(&a, &b, &intro));
    println!();
}
