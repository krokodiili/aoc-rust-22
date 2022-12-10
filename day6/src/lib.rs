
pub fn find_marker_position(datastream: &str) -> i32 {
    let chars: Vec<char> = datastream.chars().collect();
    let (index_found, _) = chars.iter().enumerate().find(|(index, character)| {
        let end_index = index + 4 as usize;
        let sequence = &chars[*index..end_index];

        println!("{:?}", sequence);
        return true;

    }).unwrap();

    return 5;     
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
