use std::collections::HashSet;
use itertools::Itertools; 

pub fn get_priority_sum_of_shared_items(sheet: String) -> i32 {
    return sheet.lines().fold(0, |acc, curr| {
        let (compartment1, compartment2) = divide_rucksack(curr);
        let compartment2_chars: HashSet<char> = compartment2.chars().collect();
        let shared_char = compartment1
            .chars()
            .find(|char| return compartment2_chars.contains(char))
            .unwrap();

        let score = give_score_to_char(shared_char);
        return acc + score;
    });
}
pub fn get_priority_sum_of_groups(sheet: String) -> i32 {
 return sheet.lines().collect::<Vec<&str>>().chunks(3).fold(0, |acc, curr| {
    //TODO:
    let (a,b,c) = curr.into_iter().try_into();
    return 42;
 }); 
}

fn divide_rucksack(rucksack: &str) -> (&str, &str) {
    return rucksack.split_at(rucksack.len() / 2);
}

fn give_score_to_char(character: char) -> i32 {
    let index_of_char = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .position(|item| item == character)
        .unwrap();

    return index_of_char as i32 + 1;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
