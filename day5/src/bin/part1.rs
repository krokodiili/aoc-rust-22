use std::fs;

use day5::execute_move;
use day5::join_tops_of_stacks;
use day5::stacks_string_to_vectors;

fn main() {
    let stacks = fs::read_to_string("stacks.txt").unwrap();
    let commands = fs::read_to_string("commands.txt").unwrap();
    let result = commands
        .lines()
        .fold(stacks_string_to_vectors(&stacks), |acc, curr| {
            return execute_move(acc, curr.to_string(), Some(false));
        });

    println!("result yeah {:?}", join_tops_of_stacks(result));
}
