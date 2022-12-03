use day1::process_part1;
use std::fs;

fn main() { 
 let read_result = fs::read_to_string("input.txt");

    let contents = match read_result {
        Ok(data) => data,
        Err(error) => panic!("Homma kusi {:?}", error)
    }; 

    let highest_load_carried = process_part1(contents);
    println!("result {}",highest_load_carried);
}