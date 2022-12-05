#![allow(non_snake_case)]
use utils::check_for_file;
pub mod d4;
pub mod utils;

fn main() {
    check_for_file("4");
    d4::day_4().unwrap();
}
