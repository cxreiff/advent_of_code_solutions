pub fn part_1(_input: &str) -> String {
    "part1".to_string()
}

pub fn part_2(_input: &str) -> String {
    "part2".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "498,4 -> 498,6 -> 496,6\n",
        "503,4 -> 502,4 -> 502,9 -> 494,9\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "part1");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "part2");
    }
}
