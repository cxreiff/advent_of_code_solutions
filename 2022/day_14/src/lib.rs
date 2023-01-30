use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .flat_map(|pair| pair.split_once(','))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .tuple_windows()
                .flat_map(|((a_x, a_y), (b_x, b_y))| {
                    let x_range = a_x.min(b_x)..=a_x.max(b_x);
                    let y_range = a_y.min(b_y)..=a_y.max(b_y);
                    x_range.cartesian_product(y_range)
                })
        })
        .collect()
}

fn find_depth(rocks: &HashSet<(usize, usize)>) -> usize {
    rocks.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1
}

fn drop_sand(mut impassable: HashSet<(usize, usize)>, depth: usize) -> usize {
    let mut sand_dropped = 0;
    loop {
        let mut sand = (500, 0);
        loop {
            sand.1 += 1;
            if sand.1 > depth {
                break;
            }
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 -= 1;
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 += 2;
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 -= 1;
            sand.1 -= 1;
            impassable.insert(sand);
            sand_dropped += 1;
            break;
        }
        if sand.1 > depth {
            break;
        }
    }
    sand_dropped
}

fn drop_sand_floored(mut impassable: HashSet<(usize, usize)>, depth: usize) -> usize {
    let mut sand_dropped = 0;
    loop {
        let mut sand = (500, 0);
        if impassable.get(&sand).is_some() {
            break;
        }
        loop {
            if sand.1 > depth {
                impassable.insert(sand);
                sand_dropped += 1;
                break;
            }
            sand.1 += 1;
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 -= 1;
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 += 2;
            if impassable.get(&sand).is_none() {
                continue;
            }
            sand.0 -= 1;
            sand.1 -= 1;
            impassable.insert(sand);
            sand_dropped += 1;
            break;
        }
    }
    sand_dropped
}

pub fn part_1(input: &str) -> String {
    let rocks = parse_input(input);
    let depth = find_depth(&rocks);
    let sand_dropped = drop_sand(rocks, depth);
    sand_dropped.to_string()
}

pub fn part_2(input: &str) -> String {
    let rocks = parse_input(input);
    let depth = find_depth(&rocks);
    let sand_dropped = drop_sand_floored(rocks, depth);
    sand_dropped.to_string()
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
        assert_eq!(part_1(INPUT), "24");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "93");
    }
}
