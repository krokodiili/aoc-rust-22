
pub fn get_highest_calories_carried(contents: String) -> i32 {
    let summed_calories = get_total_calories_list(contents);
    return *summed_calories.iter().max().expect("???");
}

pub fn get_three_highest_calories_carried_total(contents: String) -> i32 { 
    let mut summed_calories = get_total_calories_list(contents);
    summed_calories.sort_by(|a,b | b.cmp(a));

    let first_three = &summed_calories[0..3];
    return first_three.iter().sum();
}

fn get_total_calories_list(contents: String) -> Vec<i32> {
    //rsplit?
    let calories_by_person = contents.lines().fold(Vec::new(), |acc, curr| { 
        let mut clone = acc.clone();
        if clone.len() == 0 { 
            clone.push(Vec::new());
        }
        if curr == "" {
            clone.push(Vec::new());
            return clone
        }

        clone.last_mut().unwrap().push(curr.parse::<i32>().unwrap());
        return clone;
    });

    let summed_calories: Vec<i32> = calories_by_person.iter().map(|person_calories| { 
        return person_calories.iter().sum()
    }).collect();

    return summed_calories;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let stuff_to_parse = "
1
2
3

4
5
6

7
8
9".to_string();
        let result = get_highest_calories_carried(stuff_to_parse);
        assert_eq!(result, 24);
    }

    #[test]
    fn part2_works() {
        let stuff_to_parse = "1
2
3

4
5

6

7
8
9".to_string();
        let result = get_three_highest_calories_carried_total(stuff_to_parse);
        assert_eq!(result, 39);
}}
