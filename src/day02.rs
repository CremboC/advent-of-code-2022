use nom;
use nom::character::complete::{alpha1, char};
use nom::sequence::separated_pair;
use std::fs;

fn parser(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let mut parser = separated_pair(alpha1, char(' '), alpha1);
    return parser(input);
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read file.");

    let score = input.split("\n").fold(0, |score, line| {
        let (_, (l, r)) = parser(line).expect("Failed to parse");
        let add = match r {
            "X" => {
                1 + match l {
                    "A" => 3,
                    "B" => 0,
                    "C" => 6,
                    _ => 0,
                }
            }
            "Y" => {
                // paper
                2 + match l {
                    "A" => 6,
                    "B" => 3,
                    "C" => 0,
                    _ => 0,
                }
            }
            "Z" => {
                // scissors
                3 + match l {
                    "A" => 0,
                    "B" => 6,
                    "C" => 3,
                    _ => 0,
                }
            }
            _ => 0,
        };
        score + add
    });

    println!("Result {score}");
    return;
}

pub fn part2() {
    enum Hand {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    enum State {
        Lose = 0,
        Draw = 3,
        Win = 6,
    }
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
        let hand = match l {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("No hand match"),
        };
        let state = match r {
            "X" => State::Lose,
            "Y" => State::Draw,
            "Z" => State::Win,
            _ => panic!("No state match"),
        };

        let hand_to_play = get_hand(hand, &state);
        score + (hand_to_play as i32) + (state as i32)
    });

    println!("Result {score}");
    return;
}
