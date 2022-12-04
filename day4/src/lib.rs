
pub fn get_overlapping_pair_amount(sheet: String) -> i32 { 
    sheet.lines().map(|line| { 
        let parts =line.split(",");
        println!("{:?}", parts)

    });

    return 42;

}
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
