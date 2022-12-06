use itertools::Itertools;
use std::collections::HashSet;

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
    let group_sacks = sheet
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let first_sack = group[0];
            let priority = first_sack.chars().find(|char| {
                let sack2_has_it = group[1].contains(*char);
                let sack3_has_it = group[2].contains(*char);
                return sack2_has_it && sack3_has_it;
            });
            return give_score_to_char(priority.unwrap());
        })
        .collect_vec();

    return group_sacks.iter().sum();
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
