pub mod book;
pub mod side_quests;

// use crate::book::ch2_guess_game::main as ch2;
// use crate::book::ch3_basics::main as ch3;
use crate::side_quests::temperature::convert_temp;

fn main() {
    convert_temp();
}

