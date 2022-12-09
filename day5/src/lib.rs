use itertools::Itertools;
use regex::Regex;

fn strip_string_to_alphabets(stack_string: &str) -> String {
    let re = Regex::new(r"([A-Z])").unwrap();
    let result: Vec<&str> = re.find_iter(stack_string).map(|cap| cap.as_str()).collect();

    return result.join("");
}

pub fn stacks_string_to_vectors(stacks: &str) -> Vec<Vec<char>> {
    println!("{}", stacks);
    let lines: Vec<Vec<char>> = stacks
        .lines()
        .map(|line| strip_string_to_alphabets(line).chars().collect())
        .collect();

    let grouped_lines = lines.into_iter().fold(vec![vec![]], |mut acc, line| {
        line.into_iter().enumerate().for_each(|(index, char)| {
            let index_to_push = index + 1;
            if acc.len() > index_to_push {
                acc[index_to_push].push(char)
            } else {
                acc.push(vec![char])
            };
        });
        return acc;
    });

    let reversed = grouped_lines
        .into_iter()
        .map(|mut group| {
            group.reverse();
            return group;
        })
        .collect_vec();

    return reversed;
}

fn execute_move(
    crate_stacks: Vec<Vec<char>>,
    command: String,
    multilift_crane: Option<bool>,
) -> Vec<Vec<char>> {
    let (amount, from, to) = parse_move(command);
    let move_multi = multilift_crane.unwrap_or(false);

    let from_stack = crate_stacks[from as usize].clone();
    let mut to_stack = crate_stacks[to as usize].clone();

    let split_index = from_stack.len() as i32 - amount;

    let (remaining, to_move) = from_stack.split_at(split_index as usize);

    if move_multi {
        to_stack.append(&mut to_move.to_vec());
    } else {
        to_move.iter().for_each(|item| to_stack.push(*item));
    }

    let mut result = crate_stacks.clone();
    result[from as usize] = remaining.to_vec();
    result[to as usize] = to_stack;

    println!("result {:?}", to_move);

    return result;
}

fn parse_move(command: String) -> (i32, i32, i32) {
    let re = Regex::new(r"(\d)").unwrap();
    let result = re
        .find_iter(&command)
        .map(|cap| cap.as_str().parse::<i32>().unwrap())
        .into_iter();

    return result.collect_tuple().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_move_works() {
        let starting_point = vec![vec![], vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        let command = "move 2 from 1 to 3".to_string();

        let expected_result = vec![vec![], vec!['Z'], vec!['M', 'C'], vec!['P', 'N', 'D']];
        let result = execute_move(starting_point, command);
        assert_eq!(result, expected_result)
    }

    #[test]
    fn strip_stack_string_to_alphabets_works() {
        let stacks = "[Z] [M] [P]";
        let result = strip_string_to_alphabets(stacks);
        let expected_result = "ZMP";
        assert_eq!(result, expected_result)
    }

    #[test]
    fn stacks_string_to_vectors_works() {
        let stacks = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3
            ";

        let result = stacks_string_to_vectors(stacks);
        let expected_result = vec![vec![], vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        assert_eq!(result, expected_result);
    }
}
