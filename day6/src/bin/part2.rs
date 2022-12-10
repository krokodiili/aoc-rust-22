use day6::find_marker_position;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!(
        "result yeah {:?}",
        find_marker_position(&input.as_str(), 14)
    );
}
