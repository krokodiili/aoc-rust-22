use std::fs;

use day2::calculate_score_from_sheet;

fn main() {
    let sheet = fs::read_to_string("input.txt").unwrap();
    let result = calculate_score_from_sheet(sheet);

    println!("{}", result);
}
