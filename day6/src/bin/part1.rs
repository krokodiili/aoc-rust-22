use std::fs;
use day6::find_marker_position;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();


    println!("result yeah {:?}", find_marker_position(&input.as_str()));
}
