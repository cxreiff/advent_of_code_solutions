#![feature(iter_intersperse)]
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32 },
    Dir(&'a str),
}

fn nom_file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) =
        separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    Ok((input, Files::File { size }))
}

fn nom_directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn nom_ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((nom_file, nom_directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn nom_cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let operation = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, operation))
}

fn nom_commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, commands) = separated_list1(newline, alt((nom_ls, nom_cd)))(input)?;
    Ok((input, commands))
}

fn get_directory_sizes(commands: Vec<Operation>) -> BTreeMap<Vec<&str>, u32> {
    let (_, sizes) = commands.iter().fold(
        (vec![], BTreeMap::new()),
        |(mut context, mut sizes), command| {
            match command {
                Operation::Cd(Cd::Root) => {
                    context.push("");
                }
                Operation::Cd(Cd::Up) => {
                    context.pop();
                }
                Operation::Cd(Cd::Down(name)) => {
                    context.push(name);
                }
                Operation::Ls(files) => {
                    let sum = files
                        .iter()
                        .filter_map(|file| {
                            if let Files::File { size } = file {
                                Some(size)
                            } else {
                                None
                            }
                        })
                        .sum::<u32>();

                    for i in 0..context.len() {
                        sizes.entry(context[0..=i].to_vec())
                            .and_modify(|v| *v += sum)
                            .or_insert(sum);
                    }
                }
            };
            (context, sizes)
        },
    );
    sizes
}

pub fn part_1(input: &str) -> String {
    let (_, commands) = nom_commands(input).unwrap();
    let sizes = get_directory_sizes(commands);
    let summed_sizes = sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>();
    summed_sizes.to_string()
}

pub fn part_2(input: &str) -> String {
    let (_, commands) = nom_commands(input).unwrap();
    let sizes = get_directory_sizes(commands);
    let total = sizes.get(&vec![""]).unwrap();
    let target_size = total - 40000000;
    let mut sizes = sizes
        .iter()
        .map(|(_, &size)| size)
        .filter(|&size| size > target_size)
        .collect::<Vec<u32>>();
    sizes.sort();
    let deletion_size = sizes.iter().next().unwrap();
    deletion_size.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "95437");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "24933642");
    }
}
