use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha0, newline},
    multi::separated_list0,
    sequence::preceded,
    IResult, branch::alt,
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), alpha0)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), complete::u32)(input)?;
    let (input, tunnels) = preceded(
        alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve "))),
        separated_list0(tag(", "), alpha0),
    )(input)?;
    Ok((
        input,
        Valve {
            name,
            flow_rate,
            tunnels,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list0(newline, parse_valve)(input)
}

fn generate_hashmap<'a>(valves: &'a [Valve<'a>]) -> HashMap<&'a str, &Valve<'a>> {
    valves.iter().map(|valve| (valve.name, valve)).collect()
}

fn find_best_path(valves: &HashMap<&str, &Valve>, keys: Vec<&str>) -> u32 {
    if keys.len() > 15 {
        return 0;
    }

    let remaining = 30 - keys.len() as u32 * 2;
    let key = keys.last().unwrap();
    let current_valve = valves.get(key).unwrap();

    let best_path_amount = current_valve
        .tunnels
        .iter()
        .map(|key| find_best_path(valves, [keys.clone(), vec![key]].concat()))
        .max()
        .unwrap();

    if keys[..keys.len() - 1].contains(key) {
        best_path_amount
    } else {
        best_path_amount + current_valve.flow_rate * remaining
    }
}

pub fn part_1(input: &str) -> String {
    let (_, valves) = parse_input(input).unwrap();
    let valves = generate_hashmap(&valves);
    let best_path_amount = find_best_path(&valves, vec!["AA"]);
    best_path_amount.to_string()
}

pub fn part_2(input: &str) -> String {
    "part2".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA\n",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB\n",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD\n",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG\n",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH\n",
        "Valve HH has flow rate=22; tunnel leads to valve GG\n",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ\n",
        "Valve JJ has flow rate=21; tunnel leads to valve II\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "1651");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "part2");
    }
}
