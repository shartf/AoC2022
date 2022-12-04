#![allow(non_snake_case)]
use utils::check_for_file;
pub mod d2;
pub mod utils;

fn main() {
    check_for_file("2");
    d2::day_2().unwrap();
}
