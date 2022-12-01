#![allow(non_snake_case)]
use d1::day_1;
use utils::check_for_file;
pub mod d1;
pub mod utils;

fn main() {
    // download_file().unwrap();
    check_for_file("1");
    day_1().unwrap();
}
