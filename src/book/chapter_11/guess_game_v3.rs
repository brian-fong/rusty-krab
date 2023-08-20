use std::{ io, io::Write, cmp::Ordering };
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
   pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be >= 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be <= 100, got {}",
                value
            );
        }
        return Guess { value };
    } 

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[should_panic(expected = "must be <= 100")]
//     fn test_guess_too_high() {
//         Guess::new(200);
//     }
// }
