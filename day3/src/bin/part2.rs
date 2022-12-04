use std::fs;

use day3::get_priority_sum_of_groups;

fn main() {
    let sheet = fs::read_to_string("input.txt").unwrap();
    let result = get_priority_sum_of_groups(sheet);

    println!("{}", result);
}
