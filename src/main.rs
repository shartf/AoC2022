#![allow(non_snake_case)]
use utils::check_for_file;
pub mod d6;
pub mod utils;

fn main() {
    check_for_file("6");
    d6::day_6().unwrap();
}
