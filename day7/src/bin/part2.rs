use day7::calculate_dir_sizes_from_cli_history;
use day7::closest_to_minimum;
use std::fs;

fn main() {
    let disk_size = 70000000;
    let space_needed = 30000000;

    let input = fs::read_to_string("input.txt").unwrap();

    let dirs = calculate_dir_sizes_from_cli_history(&input.as_str());
    let dir_sizes = dirs.values().map(|i| i.clone()).collect::<Vec<i32>>();

    let used_space = dirs.get("/").unwrap();
    let remaining_space = disk_size - used_space;
    let min_space_to_clean = space_needed - remaining_space;

    let result = closest_to_minimum(min_space_to_clean, &dir_sizes);

    println!("jeah {:?}", result);
}
