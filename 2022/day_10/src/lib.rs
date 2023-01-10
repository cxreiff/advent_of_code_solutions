#![feature(array_chunks)]

struct DeviceState(i32, i32, Vec<i32>);

#[derive(Debug)]
enum Command {
    Add(i32),
    Noop,
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|line| match line {
            "noop" => Some(Command::Noop),
            add => add
                .split_once(' ')
                .map(|(_, amount)| Command::Add(amount.parse().ok().unwrap_or(0))),
        })
        .collect()
}

fn simulate_add(mut state: DeviceState, amount: &i32) -> DeviceState {
    state = simulate_noop(state);
    state = simulate_noop(state);
    state.1 += amount;
    state
}

fn simulate_noop(DeviceState(mut clock, x, mut result): DeviceState) -> DeviceState {
    clock += 1;
    result.push(x);
    DeviceState(clock, x, result)
}

fn simulate_screen(commands: Vec<Command>) -> Vec<i32> {
    commands
        .iter()
        .fold(DeviceState(0, 1, vec![]), |state, command| match command {
            Command::Add(amount) => simulate_add(state, amount),
            Command::Noop => simulate_noop(state),
        })
        .2
}

pub fn part_1(input: &str) -> String {
    let commands = parse_commands(input);
    let strengths = simulate_screen(commands);
    (0..6)
        .map(|tick| 20 + 40 * tick)
        .map(|clock| strengths[clock - 1] * (clock as i32))
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let commands = parse_commands(input);
    let strengths = simulate_screen(commands);
    strengths.array_chunks::<40>().map(|row| row.iter().enumerate().map(|(index, strength)| {
        if (*strength - (index as i32)).abs() < 2 {
            '#'
        } else {
            '.'
        }
    }).collect()).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    
    #[rustfmt::skip]
    const PART_2_OUTPUT: &str = concat!(
        "##..##..##..##..##..##..##..##..##..##..\n",
        "###...###...###...###...###...###...###.\n",
        "####....####....####....####....####....\n",
        "#####.....#####.....#####.....#####.....\n",
        "######......######......######......####\n",
        "#######.......#######.......#######.....",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "13140");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), PART_2_OUTPUT);
    }
}
