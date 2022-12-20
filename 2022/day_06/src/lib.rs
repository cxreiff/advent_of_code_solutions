use std::collections::HashSet;

fn find_marker(input: &str, window_size: usize) -> String {
    let chars = input.chars().collect::<Vec<char>>();

    let (index, _) = chars
        .windows(window_size)
        .enumerate()
        .find(|(_, letter)| letter.len() == letter.iter().collect::<HashSet<&char>>().len())
        .unwrap();

    (index + window_size).to_string()
}

pub fn part_1(input: &str) -> String {
    find_marker(input, 4)
}

pub fn part_2(input: &str) -> String {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
