//TODO: Proper enum to remove the unnecessary panics
//
pub fn calculate_score_from_sheet(sheet: String) -> i32 {
    return sheet
        .lines()
        .map(|line| {
            let choices = line.split(" ").collect::<Vec<&str>>();
            let enemy_choice = choices[0].chars().nth(0).unwrap();
            let my_choice = convert_my_choices(choices[1]);

            return get_round_points(my_choice, enemy_choice);
        })
        .sum();
}

pub fn calculate_score_from_game_plan(sheet: String) -> i32 {
    return sheet
        .lines()
        .map(|line| {
            let choices = line.split(" ").collect::<Vec<&str>>();
            let enemy_choice = choices[0].chars().nth(0).unwrap();
            let my_choice =
                convert_game_plan_to_choice(choices[1].chars().nth(0).unwrap(), enemy_choice);

            return get_round_points(my_choice, enemy_choice);
        })
        .sum();
}

fn convert_game_plan_to_choice(game_plan: char, enemy_choice: char) -> char {
    return match game_plan {
        'Z' => match enemy_choice {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => panic!("oops"),
        },
        'Y' => enemy_choice,
        'X' => match enemy_choice {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("oops"),
        },
        _ => panic!("oops"),
    };
}

fn get_round_points(my_choice: char, enemy_choice: char) -> i32 {
    let choice_points = match my_choice {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("oops"),
    };

    let action_points = match my_choice {
        'A' => match enemy_choice {
            'A' => 3,
            'B' => 0,
            'C' => 6,
            _ => panic!("oops"),
        },
        'B' => match enemy_choice {
            'A' => 6,
            'B' => 3,
            'C' => 0,
            _ => panic!("oops"),
        },
        'C' => match enemy_choice {
            'A' => 0,
            'B' => 6,
            'C' => 3,
            _ => panic!("oops"),
        },
        _ => panic!("oops"),
    };

    return choice_points + action_points;
}

fn convert_my_choices(choice: &str) -> char {
    return match choice {
        "X" => 'A',
        "Y" => 'B',
        "Z" => 'C',
        _ => panic!("oops"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculate_score_from_sheet;

    #[test]
    fn part1_works() {
        let input = "A Y
B X
C Z
"
        .to_string();
        let result = calculate_score_from_sheet(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let input = "A Y
B X
C Z
"
        .to_string();
        let result = calculate_score_from_game_plan(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn convert_my_choices_works() {
        let res1 = convert_my_choices("X");
        assert_eq!(res1, 'A');
        let res1 = convert_my_choices("Y");
        assert_eq!(res1, 'B');
        let res1 = convert_my_choices("Z");
        assert_eq!(res1, 'C');
    }

    #[test]
    fn get_round_points_works() {
        //TODO: Is not full coverage
        let loss = get_round_points('A', 'B');
        let tie = get_round_points('A', 'A');
        let win = get_round_points('A', 'C');
        assert_eq!(win, 7);
        assert_eq!(loss, 1);
        assert_eq!(tie, 4);
    }

    #[test]
    fn game_plan_to_choice_works() {
        //TODO: Is not full coverage
        let win = convert_game_plan_to_choice('Z', 'A');
        let tie = convert_game_plan_to_choice('Y', 'A');
        let loss = convert_game_plan_to_choice('X', 'A');

        assert_eq!(win, 'B');
        assert_eq!(tie, 'A');
        assert_eq!(loss, 'C');
    }
}
