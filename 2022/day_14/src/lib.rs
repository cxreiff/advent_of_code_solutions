use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, usize) {
    input.lines().fold((HashSet::new(), 0_usize), |(mut set, mut depth), line| {
        line.split(" -> ")
            .flat_map(|pair| pair.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .inspect(|(_, y)| depth = depth.max(*y))
            .tuple_windows()
            .for_each(|((a_x, a_y), (b_x, b_y))| {
                if a_x == b_x {
                    (a_y.min(b_y)..=a_y.max(b_y)).for_each(|y_coord| {
                        set.insert((a_x, y_coord));
                    });
                } else {
                    (a_x.min(b_x)..=a_x.max(b_x)).for_each(|x_coord| {
                        set.insert((x_coord, a_y));
                    });
                }
            });
        (set, depth)
    })
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
    let (impassable, depth) = parse_input(input);
    let sand_dropped = drop_sand(impassable, depth);
    sand_dropped.to_string()
}

pub fn part_2(input: &str) -> String {
    let (impassable, depth) = parse_input(input);
    let sand_dropped = drop_sand_floored(impassable, depth);
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
