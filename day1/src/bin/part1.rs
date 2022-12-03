use day1::get_highest_calories_carried;
use std::fs;

fn main() { 
 let read_result = fs::read_to_string("input.txt");

    let contents =read_result.unwrap(); 

    let highest_load_carried = get_highest_calories_carried(contents);
    println!("result {}",highest_load_carried);
}