use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

struct Move {
    amount: usize,
    origin: usize,
    destination: usize,
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, crated) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match crated {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn parse_row(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, result))
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, stacks_horizontal) = separated_list1(newline, parse_row)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let mut stacks_vertical = vec![];
    for _ in 0..=stacks_horizontal.len() {
        stacks_vertical.push(vec![]);
    }
    for vec in stacks_horizontal.iter().rev() {
        for (i, crated) in vec.iter().enumerate() {
            if crated.is_some() {
                stacks_vertical[i].push(crated.unwrap().clone());
            }
        }
    }

    Ok((input, stacks_vertical))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, origin) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, destination) = complete::u32(input)?;

    Ok((
        input,
        Move {
            amount: amount as usize,
            origin: (origin - 1) as usize,
            destination: (destination - 1) as usize,
        },
    ))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, result) = separated_list1(newline, parse_move)(input)?;

    Ok((input, result))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, stacks) = parse_stacks(input)?;
    let (input, moves) = parse_moves(input)?;

    Ok((input, (stacks, moves)))
}

pub fn part_1(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();
    for Move {
        amount,
        origin,
        destination,
    } in moves.iter()
    {
        let origin_len = stacks[*origin].len();
        let to_move = stacks[*origin]
            .drain((origin_len - *amount)..)
            .rev()
            .collect::<Vec<_>>();
        stacks[*destination].extend(to_move);
    }

    stacks
        .iter()
        .map(|stack| match stack.iter().last() {
            Some(crated) => crated,
            None => "",
        })
        .collect::<String>()
}

pub fn part_2(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();
    for Move {
        amount,
        origin,
        destination,
    } in moves.iter()
    {
        let origin_len = stacks[*origin].len();
        let to_move = stacks[*origin]
            .drain((origin_len - *amount)..)
            .collect::<Vec<_>>();
        stacks[*destination].extend(to_move);
    }

    stacks
        .iter()
        .map(|stack| match stack.iter().last() {
            Some(crated) => crated,
            None => "",
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "           \n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "CMZ");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "MCD");
    }
}
