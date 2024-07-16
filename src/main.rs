pub mod utils;
pub mod aoc_2021;

use utils::file_loader::load_file;

fn main() {
    let mut alias_data = load_file("/Users/mac-m3/.alias");
    let first_line = alias_data.next().expect("atleast a line");
    println!("{first_line}");
}
