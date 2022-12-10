pub fn find_marker_position(datastream: &str) -> i32 {
    let chars: Vec<char> = datastream.chars().collect();
    let (index_found, _) = chars
        .iter()
        .enumerate()
        .find(|(index, character)| {
            let end_index = index + 4 as usize;
            let sequence = &chars[*index..end_index];
            let seq_string: String = sequence.iter().collect();

            let char_counts = sequence.iter().find(|&&character| {
                return seq_string.matches(character).count() > 1;
            });

            return match char_counts {
                Some(_) => false,
                _ => true,
            };
        })
        .unwrap();

    return index_found as i32 + 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
