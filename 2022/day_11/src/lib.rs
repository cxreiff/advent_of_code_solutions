use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::{complete::tag, streaming::take_until},
    character::complete::{self, alphanumeric1, char, multispace0, newline},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square(),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    if_true_monkey: u64,
    if_false_monkey: u64,
    inspections: u64,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (operation, rhs)) =
        separated_pair(alt((char('+'), char('*'))), multispace0, alphanumeric1)(input)?;
    Ok((
        input,
        match rhs {
            "old" => match operation {
                '+' => Operation::Multiply(2),
                '*' => Operation::Square(),
                _ => panic!(),
            },
            _ => match operation {
                '+' => Operation::Add(rhs.parse().unwrap()),
                '*' => Operation::Multiply(rhs.parse().unwrap()),
                _ => panic!(),
            },
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = take_until("items: ")(input)?;
    let (input, _) = tag("items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), complete::u64)(input)?;
    let (input, _) = take_until("old ")(input)?;
    let (input, _) = tag("old ")(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = take_until("divisible by ")(input)?;
    let (input, _) = tag("divisible by ")(input)?;
    let (input, test_divisor) = complete::u64(input)?;
    let (input, _) = take_until("monkey ")(input)?;
    let (input, _) = tag("monkey ")(input)?;
    let (input, if_true_monkey) = complete::u64(input)?;
    let (input, _) = take_until("monkey ")(input)?;
    let (input, _) = tag("monkey ")(input)?;
    let (input, if_false_monkey) = complete::u64(input)?;
    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation,
            test_divisor,
            if_true_monkey,
            if_false_monkey,
            inspections: 0,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(count(newline, 2), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn simulate_keep_away<F>(monkeys: &mut Vec<Monkey>, rounds: usize, inspection_callback: F)
where
    F: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys[i].clone();
            for j in 0..current_monkey.items.len() {
                let mut item = current_monkey.items[j];
                match current_monkey.operation {
                    Operation::Add(ref amount) => item += amount,
                    Operation::Multiply(ref amount) => item *= amount,
                    Operation::Square() => item *= item,
                }
                item = inspection_callback(item);
                if item % current_monkey.test_divisor == 0 {
                    monkeys
                        .get_mut(current_monkey.if_true_monkey as usize)
                        .unwrap()
                        .items
                        .push_back(item);
                } else {
                    monkeys
                        .get_mut(current_monkey.if_false_monkey as usize)
                        .unwrap()
                        .items
                        .push_back(item);
                }
                monkeys.get_mut(i).unwrap().inspections += 1;
            }
            monkeys.get_mut(i).unwrap().items.clear();
        }
    }
}

fn sum_two_most_active(monkeys: &[Monkey]) -> u64 {
    let (first, second) = monkeys.iter().fold((0, 0), |(first, second), monkey| {
        if first < monkey.inspections {
            return (monkey.inspections, first);
        }

        if second < monkey.inspections {
            return (first, monkey.inspections);
        }

        (first, second)
    });
    first * second
}

pub fn part_1(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();
    simulate_keep_away(&mut monkeys, 20, |item| item / 3);
    sum_two_most_active(&monkeys).to_string()
}

pub fn part_2(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();
    let test_product = monkeys
        .iter()
        .map(|Monkey { test_divisor, .. }| test_divisor)
        .product::<u64>();
    simulate_keep_away(&mut monkeys, 10000, |item| item % test_product);
    sum_two_most_active(&monkeys).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "10605");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "2713310158");
    }
}
