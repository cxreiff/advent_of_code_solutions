fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.split("\n\n").map(|calorie_counts| {
        calorie_counts
            .lines()
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum::<u32>()
    })
}

pub fn part_1(input: &str) -> String {
    parse_input(input).max().unwrap().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut calorie_sums = parse_input(input).collect::<Vec<_>>();

    calorie_sums.sort_by(|a, b| b.cmp(a));

    calorie_sums.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "24000");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "45000");
    }
}
