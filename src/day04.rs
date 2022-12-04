use nom;
use nom::character::complete::{char, digit1};
use nom::sequence::separated_pair;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

struct Pair(RangeInclusive<u8>, RangeInclusive<u8>);
impl Pair {
    fn create(s: ((&str, &str), (&str, &str))) -> Pair {
        let ((s1, e1), (s2, e2)) = s;
        Pair(
            RangeInclusive::new(s1.parse().unwrap(), e1.parse().unwrap()),
            RangeInclusive::new(s2.parse().unwrap(), e2.parse().unwrap()),
        )
    }
}

fn parser(input: &str) -> nom::IResult<&str, ((&str, &str), (&str, &str))> {
    return separated_pair(
        separated_pair(digit1, char('-'), digit1),
        char(','),
        separated_pair(digit1, char('-'), digit1),
    )(input);
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read file.");
    let result = input
        .split("\n")
        .map(|line| {
            let (_, parsed) = parser(line).expect("Invalid line!");
            let pair = Pair::create(parsed);

            let left_set = HashSet::<u8>::from_iter(pair.0.into_iter());
            let right_set = HashSet::<u8>::from_iter(pair.1.into_iter());

            if left_set.is_subset(&right_set) || right_set.is_subset(&left_set) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("D4P1 is {:?}", result)
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read file.");
    let result = input
        .split("\n")
        .map(|line| {
            let (_, parsed) = parser(line).expect("Invalid line!");
            let pair = Pair::create(parsed);

            let left_set = HashSet::<u8>::from_iter(pair.0.into_iter());
            let right_set = HashSet::<u8>::from_iter(pair.1.into_iter());

            if left_set.intersection(&right_set).count() > 0 {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("D4P2 is {:?}", result)
}
