use std::fs;

use day4::get_covered_pair_amount;

fn main() {
    let sheet = fs::read_to_string("input.txt").unwrap();
    let result = get_covered_pair_amount(sheet);

    println!("{}", result);
}
