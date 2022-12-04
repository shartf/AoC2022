#![allow(non_snake_case)]
use utils::check_for_file;
pub mod d3;
pub mod utils;

fn main() {
    check_for_file("3");
    d3::day_3().unwrap();
}
