use std::collections::HashSet;
use std::fs;

fn search(w: usize) -> usize {
    let input = fs::read("inputs/day06.txt").expect("Failed to read file.");
    let mut result = w - 1;
    for window in input.windows(w) {
        result += 1;
        let set = HashSet::<u8>::from_iter(window.iter().map(|x| *x));
        if set.len() == w {
            break;
        }
    }
    result
}

pub fn part1() {
    let result = search(4);
    println!("D06P1 {:?}", result);
}

pub fn part2() {
    let result = search(14);
    println!("D06P2 {:?}", result);
}
