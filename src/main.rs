#![allow(dead_code)]
#![allow(unused_imports)]

mod book;
mod side_quests;

use crate::book::chapter_2::guess_game;
use crate::book::chapter_3::basics;
use crate::book::chapter_4::ownership;
use crate::book::chapter_5::structs;
use crate::book::chapter_6::enums;
use crate::book::chapter_6::patterns;
use crate::book::chapter_8::vectors;
use crate::book::chapter_8::strings;
use crate::book::chapter_8::hashmaps;
use crate::book::chapter_9::errors;
use crate::book::chapter_9::guess_game_v2;


fn main() {
    guess_game_v2::main();
}

