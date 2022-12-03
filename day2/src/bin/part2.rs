use std::fs;

use day2::calculate_score_from_game_plan;

fn main() {
    let sheet = fs::read_to_string("input.txt").unwrap();
    let result = calculate_score_from_game_plan(sheet);

    println!("{}", result);
}
