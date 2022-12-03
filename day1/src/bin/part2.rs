use day1::get_three_highest_calories_carried_total;
use std::fs;

fn main() { 
 let read_result = fs::read_to_string("input.txt");

    let contents =  read_result.unwrap(); 

    let highest_loads_carried = get_three_highest_calories_carried_total(contents);
    println!("result {}",highest_loads_carried);
}