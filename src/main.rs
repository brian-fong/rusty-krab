#![allow(dead_code)]
#![allow(unused_imports)]

mod book;
mod side_quests;

use crate::book::chapter_02::guess_game;
use crate::book::chapter_03::basics;
use crate::book::chapter_04::ownership;
use crate::book::chapter_05::structs;
use crate::book::chapter_06::enums;
use crate::book::chapter_06::patterns;
use crate::book::chapter_08::vectors;
use crate::book::chapter_08::strings;
use crate::book::chapter_08::hashmaps;
use crate::book::chapter_09::errors;
use crate::book::chapter_09::guess_game_v2;
use crate::book::chapter_10::duplicate;
use crate::book::chapter_10::generics;
use crate::book::chapter_10::traits;
use crate::book::chapter_10::lifetimes;

fn main() {
    lifetimes::main();
}
