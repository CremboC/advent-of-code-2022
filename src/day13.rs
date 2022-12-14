use itertools::{EitherOrBoth, Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;
use std::io::BufRead;

fn is_ordered(left: &Value, right: &Value) -> Ord {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => {
            if a < b {
                Ord::True
            } else if a == b {
                Ord::Unknown
            } else {
                Ord::False
            }
        }
        (Value::List(_), Value::Int(b)) => {
            let value = Value::List(Box::new(vec![Value::Int(*b)]));
            is_ordered(left, &value)
        }
        (Value::Int(a), Value::List(_)) => {
            let value = Value::List(Box::new(vec![Value::Int(*a)]));
            is_ordered(&value, right)
        }
        (Value::List(a), Value::List(b)) => {
            for pair in a.iter().zip_longest(b.iter()) {
                match pair {
                    EitherOrBoth::Both(el_a, el_b) => match is_ordered(el_a, el_b) {
                        Ord::True => {
                            return Ord::True;
                        }
                        Ord::False => {
                            return Ord::False;
                        }
                        Ord::Unknown => {
                            continue;
                        }
                    },
                    EitherOrBoth::Left(_) => {
                        // only have elements on left, right list has ran out
                        return Ord::False;
                    }
                    EitherOrBoth::Right(_) => {
                        // only have elements on the right, left list has ran out
                        return Ord::True;
                    }
                }
            }

            Ord::Unknown
        }
    }
}

pub fn part1() {
    let input = get_input();
    let mut result = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        let ordered = is_ordered(&left, &right);
        if ordered == Ord::True {
            result += i + 1;
        }
    }
    println!("D13P1 {}", result);
}

pub fn part2() {
    let div1 = Value::List(Box::new(vec![Value::Int(2)]));
    let div2 = Value::List(Box::new(vec![Value::Int(6)]));

    let input = get_input();
    let mut signals: Vec<_> = input.iter().flat_map(|(l, r)| vec![l, r]).collect();
    signals.push(&div1);
    signals.push(&div2);
    signals.sort();

    let div1_loc = signals
        .iter()
        .enumerate()
        .find_map(|(i, &v)| if v.eq(&div1) { Some(i) } else { None })
        .unwrap()
        + 1;
    let div2_loc = signals
        .iter()
        .enumerate()
        .find_map(|(i, &v)| if v.eq(&div2) { Some(i) } else { None })
        .unwrap()
        + 1;

    println!("D13P2 {}", div1_loc * div2_loc);
}

fn get_input() -> Vec<(Value, Value)> {
    include_bytes!("../inputs/day13.txt")
        .lines()
        .filter_map(|l| l.ok())
        .chunks(3)
        .into_iter()
        .map(|pair| {
            let pair_: Vec<String> = pair.collect();
            let (_, left) = parse(pair_[0].as_ref()).unwrap();
            let (_, right) = parse(pair_[1].as_ref()).unwrap();
            (left, right)
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Value> {
    alt((parse_list, parse_el))(input)
}

fn parse_el(input: &str) -> IResult<&str, Value> {
    map(digit1, |d: &str| Value::Int(d.parse::<u8>().unwrap()))(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    map(
        delimited(tag("["), separated_list0(tag(","), parse), tag("]")),
        |l: Vec<Value>| Value::List(Box::new(l)),
    )(input)
}

#[derive(Debug)]
enum Value {
    Int(u8),
    List(Box<Vec<Value>>),
}

impl Eq for Value {}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::List(a), Value::List(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_ordered(self, other) {
            Ord::False => Ordering::Greater,
            Ord::True => Ordering::Less,
            Ord::Unknown => Ordering::Equal,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Ord {
    False,
    True,
    Unknown,
}
