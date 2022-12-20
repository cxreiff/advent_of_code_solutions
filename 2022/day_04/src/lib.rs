fn parse_starts_and_ends(input: &str) -> impl Iterator<Item = Vec<Vec<u32>>> + '_ {
    input.lines().map(|line| {
        line.split(',')
            .map(|elf| {
                elf.split('-')
                    .map(|id| id.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
    })
}

pub fn part_1(input: &str) -> String {
    parse_starts_and_ends(input)
        .filter(|line| {
            (line[0][0] <= line[1][0] && line[0][1] >= line[1][1])
                || (line[0][0] >= line[1][0] && line[0][1] <= line[1][1])
        })
        .count()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    parse_starts_and_ends(input)
        .filter(|line| !(line[0][1] < line[1][0] || line[0][0] > line[1][1]))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "2");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "4");
    }
}
