use crate::day07::cmd::Cd;
use itertools::Itertools;
use nom::InputIter;
use std::collections::HashMap;
use std::ops::AddAssign;

pub fn part1() {
    let sizes = calculate_sizes();
    let result: u32 = sizes
        .values()
        .filter_map(|&x| if x <= 100_000 { Some(x) } else { None })
        .sum();

    println!("D07P1 {:?}", result);
}

pub fn part2() {
    const TOTAL_SIZE: u32 = 70_000_000;
    const NEEDED: u32 = 30_000_000;

    let sizes = calculate_sizes();
    let used_space = sizes.values().max().expect("Must be a max");
    let missing = NEEDED - (TOTAL_SIZE - used_space);

    let (_, result) = sizes
        .iter()
        .sorted_by_key(|(_, &y)| y)
        .find(|(_, &y)| y >= missing)
        .unwrap();

    println!("D07P2 {:?}", result);
}

mod cmd {
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_till};
    use nom::character::complete::{alpha1, digit1, space1};
    use nom::combinator::map;
    use nom::sequence::separated_pair;
    use nom::IResult;

    pub enum Cd {
        Real(String),
        Back,
        Home,
    }

    pub struct File {
        pub size: u32,
    }

    pub enum Cmd {
        Cd(Cd),
        File(File),
    }

    fn parse_cd() -> impl FnMut(&str) -> IResult<&str, Cmd> {
        move |i| {
            map(
                separated_pair(
                    tag("cd"),
                    space1,
                    alt((
                        map(alpha1, |c: &str| Cd::Real(String::from(c))),
                        map(tag(".."), |_| Cd::Back),
                        map(tag("/"), |_| Cd::Home),
                    )),
                ),
                |(_, cd)| Cmd::Cd(cd),
            )(i)
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Cmd> {
        alt((
            map(separated_pair(tag("$"), space1, parse_cd()), |(_, cmd)| cmd),
            map(
                separated_pair(digit1::<&str, _>, space1, take_till(|_| true)),
                |(size, _)| {
                    Cmd::File(File {
                        size: size.parse().unwrap(),
                    })
                },
            ),
        ))(input)
    }
}

fn calculate_sizes() -> HashMap<String, u32> {
    let cmds: Vec<_> = String::from_utf8_lossy(include_bytes!("../inputs/day07.txt"))
        .lines()
        .filter(|line| {
            line.starts_with("$ cd")
                || line.iter_elements().take_while(|c| c.is_numeric()).count() > 0
        })
        .filter_map(|line| cmd::parse(line).map(|(_, cmd)| cmd).ok())
        .collect();

    fn vec_path_to_strings(v: &Vec<String>) -> Vec<String> {
        let mut out = Vec::with_capacity(v.len() + 1);
        out.push(String::from(""));
        for i in 1..=v.len() {
            out.push(v[0..i].join("/"))
        }
        out
    }

    const BASE: Vec<String> = vec![];
    let mut current_path: Vec<String> = vec![];
    let mut sizes = HashMap::<String, u32>::new();
    for cmd in cmds {
        match cmd {
            cmd::Cmd::Cd(cd) => match cd {
                Cd::Real(dir) => current_path.push(dir),
                Cd::Back => {
                    if current_path.len() > 0 {
                        current_path.pop();
                    }
                }
                Cd::Home => current_path = BASE,
            },
            cmd::Cmd::File(file) => {
                let current_path_strings = vec_path_to_strings(&current_path);
                for path in current_path_strings {
                    sizes
                        .entry(path)
                        .and_modify(|s| s.add_assign(file.size))
                        .or_insert(file.size);
                }
            }
        }
    }
    sizes
}
