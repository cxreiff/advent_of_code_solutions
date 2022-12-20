#![feature(iter_array_chunks)]

use std::collections::HashMap;

fn generate_char_priorities() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, letter)| (letter, index + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn part_1(input: &str) -> String {
    let char_priorities = generate_char_priorities();

    input
        .lines()
        .map(|rucksack| {
            let compartment_size = rucksack.len() / 2;
            let first_compartment = &rucksack[0..compartment_size];
            let second_compartment = &rucksack[compartment_size..rucksack.len()];
            char_priorities
                .get(
                    &(first_compartment
                        .chars()
                        .find(|letter| second_compartment.contains(*letter))
                        .unwrap()),
                )
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let char_priorities = generate_char_priorities();
    input
        .lines()
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            char_priorities
                .get(
                    &(first
                        .chars()
                        .find(|letter| second.contains(*letter) & third.contains(*letter))
                        .unwrap()),
                )
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw\
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "157");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "70");
    }
}
