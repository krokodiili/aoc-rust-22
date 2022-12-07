use regex::Regex;

fn strip_string_to_alphabets(stack_string: &str) -> String {
    let re = Regex::new(r"[A-Z]").unwrap();
    let result = re.captures(stack_string).unwrap();
    println!("RESULT {:?}", result);
    return "".to_string();
}

pub fn stacks_string_to_vectors(stacks: &str) -> Vec<Vec<char>> {
    println!("{}", stacks);
    let mut lines: Vec<Vec<char>> = stacks
        .lines()
        .map(|line| strip_string_to_alphabets(line).chars().collect())
        .collect();

    //Remove bottom numbering
    lines.pop();

    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_stack_string_to_alphabets_works() {
        let stacks = "[Z] [M] [P]";
        strip_string_to_alphabets(stacks);
        let expected_result = "ZMP";
        assert_eq!(stacks, expected_result)
    }

    #[test]
    fn stacks_string_to_vectors_works() {
        let stacks = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3
            ";

        let result = stacks_string_to_vectors(stacks);
        let expected_result = vec![vec![], vec!['D', 'N', 'Z'], vec!['P']];
        assert_eq!(result, expected_result);
    }
}
