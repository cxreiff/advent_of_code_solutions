pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|round| (round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()))
        .map(|(theirs, mine)| match theirs {
            'A' => match mine {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => 0,
            },
            'B' => match mine {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match mine {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|round| (round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()))
        .map(|(theirs, mine)| match theirs {
            'A' => match mine {
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => 0,
            },
            'B' => match mine {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match mine {
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "15");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "12");
    }
}
