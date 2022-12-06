#![allow(non_snake_case)]
use utils::check_for_file;
pub mod d5;
pub mod utils;

fn main() {
    check_for_file("5");
    d5::day_5().unwrap();
}
