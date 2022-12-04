use std::cmp::*;

pub fn get_covered_pair_amount(sheet: String) -> i32 {
    return sheet.lines().fold(0, |acc, line| {
        let (start1, end1, start2, end2) = line_to_tuple(line);

        let should_increment = match start1.cmp(&start2) {
            Ordering::Equal => true,
            Ordering::Less => match end1.cmp(&end2) {
                Ordering::Equal => true,
                Ordering::Greater => true,
                Ordering::Less => false,
            },
            Ordering::Greater => match end2.cmp(&end1) {
                Ordering::Equal => true,
                Ordering::Greater => true,
                Ordering::Less => false,
            },
        };

        let increment_amount = match should_increment {
            true => 1,
            false => 0,
        };

        return acc + increment_amount;
    });
}

pub fn get_overlapping_pair_amount(sheet: String) -> i32 {
    return sheet.lines().fold(0, |acc, line| {
        let (start1, end1, start2, end2) = line_to_tuple(line);

        let should_increment = start1 <= end2 && start2 <= end1;

        let increment_amount = match should_increment {
            true => 1,
            false => 0,
        };
        return acc + increment_amount;
    });
}

fn line_to_tuple(line: &str) -> (i32, i32, i32, i32) {
    let parts = line.split(",").collect::<Vec<&str>>();
    let (start1, end1) = tuple_from_dash_string(parts[0]);
    let (start2, end2) = tuple_from_dash_string(parts[1]);
    return (start1, end1, start2, end2);
}

fn tuple_from_dash_string(value: &str) -> (i32, i32) {
    let dash_split = value.split("-").collect::<Vec<&str>>();
    return (
        dash_split[0].parse::<i32>().unwrap(),
        dash_split[1].parse::<i32>().unwrap(),
    );
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
