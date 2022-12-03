struct Points { 
    X: i32,
    Y: i32,
    Z: i32,
    LOSS: i32,
    TIE: i32,
    WIN: i32,
}

// let points_by_type = Points {
//     X: 1,
//     Y: 2,
//     Z: 3,
//     LOSS: 0,
//     TIE: 3
//     WIN: 6
// };

pub fn calculate_score_from_sheet(sheet: String) -> i32 {
    let scores = sheet.lines().map(|line| {
    let [a,b] = line.split(" ").collect::<String[]>(); 
    println!("{}", a);
    println!("{}", b);


    }).collect();

    return 42;
}

#[cfg(test)]
mod tests {
    use crate::calculate_score_from_sheet;

    #[test]
    fn it_works() {
        let input = "A Y
B X
C Z
".to_string();
        let result = calculate_score_from_sheet(input);

        assert_eq!(result, 4);
    }
}
