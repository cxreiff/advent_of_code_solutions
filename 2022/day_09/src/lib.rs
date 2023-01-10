use std::collections::HashSet;
use std::iter;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Invalid,
}

fn simulate_rope(input: &str, knots: usize) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(direction, distance)| {
                    (
                        match direction {
                            "U" => Direction::Up,
                            "D" => Direction::Down,
                            "R" => Direction::Right,
                            "L" => Direction::Left,
                            _ => Direction::Invalid,
                        },
                        distance.parse().unwrap_or(0),
                    )
                })
                .map(|(direction, distance)| iter::repeat(direction).take(distance))
        })
        .flatten()
        .fold(
            ((0_i32, 0_i32), vec![(0, 0); knots], HashSet::from([(0, 0)])),
            |(mut head, mut tails, mut visits), direction| {
                match direction {
                    Direction::Up => head.1 += 1,
                    Direction::Down => head.1 -= 1,
                    Direction::Right => head.0 += 1,
                    Direction::Left => head.0 -= 1,
                    Direction::Invalid => (),
                };
                let mut prev_knot = &head;
                for mut tail in tails.iter_mut() {
                    let diff = (prev_knot.0 - tail.0, prev_knot.1 - tail.1);
                    if diff.0.abs() > 1 || diff.1.abs() > 1 {
                        tail.0 += diff.0.signum();
                        tail.1 += diff.1.signum();
                    }
                    prev_knot = tail;
                }
                visits.insert(*prev_knot);
                (head, tails, visits)
            },
        )
        .2
        .len()
}

pub fn part_1(input: &str) -> String {
    simulate_rope(input, 1).to_string()
}

pub fn part_2(input: &str) -> String {
    simulate_rope(input, 9).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT_1: &str = concat!(
        "R 4\n",
        "U 4\n",
        "L 3\n",
        "D 1\n",
        "R 4\n",
        "D 1\n",
        "L 5\n",
        "R 2\n",
    );

    #[rustfmt::skip]
    const INPUT_2: &str = concat!(
        "R 5\n",
        "U 8\n",
        "L 8\n",
        "D 3\n",
        "R 17\n",
        "D 10\n",
        "L 25\n",
        "U 20\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT_1), "13");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT_1), "1");
        assert_eq!(part_2(INPUT_2), "36");
    }
}
