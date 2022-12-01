use std::fs;

fn main() {
    let read_result = fs::read_to_string("src/day1/input");

    let contents = match read_result {
        Ok(data) => data,
        Err(error) => panic!("Homma kusi {:?}", error)
    }; 

    let split_new_line = contents.lines()


    println!("stuff \n {}", contents);
}