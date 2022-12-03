use nom::InputIter;
use std::collections::HashSet;
use std::fs;

fn char_to_score(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        (((c as u8) - b'A' + 1) + 26) as i32
    } else {
        ((c as u8) - b'a' + 1) as i32
    }
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read file.");
    let result: i32 = input
        .split("\n")
        .map(|line| {
            let middle = line.len() / 2;
            let (start, end) = line.split_at(middle);

            let start = HashSet::<char>::from_iter(start.iter_elements());
            let end = HashSet::<char>::from_iter(end.iter_elements());
            let sum: i32 = start
                .iter()
                .map(|char| {
                    if end.contains(char) {
                        char_to_score(*char)
                    } else {
                        0
                    }
                })
                .sum();

            sum
        })
        .sum();

    println!("Result is {result}");
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read file.");
    let vec: Vec<_> = input.split("\n").collect();
    let result: i32 = vec
        .chunks(3)
        .map(|chunks| {
            let lines: Vec<HashSet<char>> = chunks
                .iter()
                .map(|line| HashSet::from_iter(line.iter_elements()))
                .collect();
            let (a, b, c) = if let [a, b, c] = lines.as_slice() {
                (a, b, c)
            } else {
                panic!("Invalid chunk")
            };
            let sum: i32 = a
                .iter()
                .map(|char| {
                    if b.contains(char) && c.contains(char) {
                        char_to_score(*char)
                    } else {
                        0
                    }
                })
                .sum();

            sum
        })
        .sum();
    println!("Result is {result}");
}
