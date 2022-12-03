
pub fn process_part1(contents: String) -> i32 {
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

    return *summed_calories.iter().max().expect("???");
}
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
