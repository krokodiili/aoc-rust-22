use day7::calculate_dir_sizes_from_cli_history;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result: i32 = calculate_dir_sizes_from_cli_history(&input.as_str())
        .values()
        .filter(|folder_size| {
            return **folder_size <= 100000;
        })
        .map(|i| i.clone())
        .sum();

    println!("jeah {:?}", result);
}
