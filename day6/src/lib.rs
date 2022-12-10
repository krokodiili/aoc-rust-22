pub fn find_marker_position(datastream: &str, marker_length: i32) -> i32 {
    let chars: Vec<char> = datastream.chars().collect();
    let (index_found, _) = chars
        .iter()
        .enumerate()
        .find(|(index, _)| {
            let end_index = index + marker_length as usize;
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

    return index_found as i32 + marker_length;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            find_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
