use std::collections::HashSet;

type Rock = Vec<(u32, u32)>;

enum EndState {
    Landed(u32),
    Falling(Rock),
}

enum Jet {
    Left,
    Right,
}

fn generate_rock(shape_number: usize, height: u32) -> Rock {
    match shape_number {
        0 => vec![(3, height), (4, height), (5, height), (6, height)],
        1 => vec![(4, height), (3, height+1), (4, height+1), (5, height+1), (4, height+2)],
        2 => vec![(3, height), (4, height), (5, height), (5, height+1), (5, height+2)],
        3 => vec![(3, height), (3, height+1), (3, height+2), (3, height+3)],
        4 => vec![(3, height), (4, height), (3, height+1), (4, height+1)],
        _ => panic!(),
    }
}

fn jet_step(board: &HashSet<(u32, u32)>, rock: Rock, direction: Jet) -> Rock {
    let new_rock: Vec<(u32, u32)> = match direction {
        Jet::Left => rock.iter().map(|(x, y)| (x-1, *y)).collect(),
        Jet::Right => rock.iter().map(|(x, y)| (x+1, *y)).collect(),
    };
    if new_rock.iter().any(|piece| board.contains(piece) || piece.0 == 0 || piece.0 == 8) {
        return rock;
    }
    new_rock
}

fn fall_step(board: &mut HashSet<(u32, u32)>, rock: Rock) -> EndState {
    let new_rock: Vec<(u32, u32)> = rock.iter().map(|(x, y)| (*x, y-1)).collect();
    if new_rock.iter().any(|piece| board.contains(piece) || piece.1 == 0) {
        rock.iter().for_each(|piece| { board.insert(*piece); });
        return EndState::Landed(*rock.iter().map(|(_, y)| y).max().unwrap());
    }
    EndState::Falling(new_rock)
}

fn simulate_motion(input: &str, rock_count: usize) -> u32 {
    let mut height = 0;
    let mut jets = input.chars().filter_map(|char| match char {
        '<' => Some(Jet::Left),
        '>' => Some(Jet::Right),
        _ => None,
    }).cycle();
    let mut board = HashSet::new();
    (0..=4).cycle().take(rock_count).for_each(|shape_number| {
        let mut rock = generate_rock(shape_number, height + 4);
        loop {
            rock = jet_step(&board, rock, jets.next().unwrap());
            match fall_step(&mut board, rock) {
                EndState::Falling(new_rock) => {
                    rock = new_rock;
                },
                EndState::Landed(new_height) => {
                    height = height.max(new_height);
                    break;
                }
            }
        }
        println!("{height}");
    });
    height
}

pub fn part_1(input: &str) -> String {
    simulate_motion(input, 2022).to_string()
}

pub fn part_2(input: &str) -> String {
    simulate_motion(input, 1000000000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "3068");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "1514285714288");
    }
}
