use nom;
use nom::character::complete::{alpha1, char};
use nom::sequence::separated_pair;
use std::fs;

#[derive(Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn parse(s: &str) -> Hand {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("No hand match"),
        }
    }
}

enum State {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl State {
    fn parse(s: &str) -> State {
        match s {
            "X" => State::Lose,
            "Y" => State::Draw,
            "Z" => State::Win,
            _ => panic!("No state match"),
        }
    }
}

fn parser(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let mut parser = separated_pair(alpha1, char(' '), alpha1);
    return parser(input);
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read file.");

    fn play(h1: Hand, h2: Hand) -> State {
        match (h1, h2) {
            (Hand::Paper, Hand::Rock) => State::Win,
            (Hand::Rock, Hand::Scissors) => State::Win,
            (Hand::Scissors, Hand::Paper) => State::Win,
            (Hand::Rock, Hand::Paper) => State::Lose,
            (Hand::Scissors, Hand::Rock) => State::Lose,
            (Hand::Paper, Hand::Scissors) => State::Lose,
            (_, _) => State::Draw,
        }
    }

    let score = input.split("\n").fold(0, |score, line| {
        let (_, (l, r)) = parser(line).expect("Failed to parse");
        let left_hand = Hand::parse(l);
        let right_hand = Hand::parse(r);
        score + (right_hand as i32) + (play(right_hand, left_hand) as i32)
    });

    println!("Result {score}");
    return;
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read file.");

    fn get_hand(h: Hand, s: &State) -> Hand {
        match (h, s) {
            (Hand::Rock, State::Lose) => Hand::Scissors,
            (Hand::Rock, State::Win) => Hand::Paper,
            (Hand::Paper, State::Lose) => Hand::Rock,
            (Hand::Paper, State::Win) => Hand::Scissors,
            (Hand::Scissors, State::Lose) => Hand::Paper,
            (Hand::Scissors, State::Win) => Hand::Rock,
            (h, State::Draw) => h,
        }
    }

    let score = input.split("\n").fold(0, |score, line| {
        let (_, (l, r)) = parser(line).expect("Failed to parse");
        let hand = Hand::parse(l);
        let state = State::parse(r);
        let hand_to_play = get_hand(hand, &state);
        score + (hand_to_play as i32) + (state as i32)
    });

    println!("Result {score}");
    return;
}
