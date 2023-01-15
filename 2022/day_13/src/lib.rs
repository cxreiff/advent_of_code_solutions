use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Element(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::Element(r0)) => l0.cmp(&vec![Packet::Element(*r0)]),
            (Self::Element(l0), Self::List(r0)) => vec![Packet::Element(*l0)].cmp(r0),
            (Self::Element(l0), Self::Element(r0)) => l0.cmp(r0),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Element(l0), Self::Element(r0)) => l0 == r0,
            (Self::List(l0), Self::Element(r0)) => *l0 == vec![Packet::Element(*r0)],
            (Self::Element(l0), Self::List(r0)) => vec![Packet::Element(*l0)] == *r0,
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        nom::character::complete::u32.map(Packet::Element),
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")).map(Packet::List),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        pair(newline, newline),
        separated_pair(parse_packet, newline, parse_packet),
    )(input)
}

pub fn part_1(input: &str) -> String {
    let (_, pairs) = parse_input(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .map(|(i, (packet1, packet2))| match packet1.cmp(packet2) {
            std::cmp::Ordering::Less => i + 1,
            std::cmp::Ordering::Equal => panic!("Pairs must have an ordering."),
            std::cmp::Ordering::Greater => 0,
        })
        .sum::<usize>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let (_, pairs) = parse_input(input).unwrap();
    
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Element(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Element(6)])]);

    let mut packets = pairs
        .iter()
        .flat_map(|(packet1, packet2)| [packet1, packet2])
        .chain([&divider1, &divider2])
        .collect::<Vec<&Packet>>();

    packets.sort();

    packets.iter().enumerate().filter_map(|(i, &packet)|
        if packet == &divider1 || packet == &divider2 {
            Some(i+1)
        } else {
            None
        }
    ).product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "13");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "140");
    }
}
