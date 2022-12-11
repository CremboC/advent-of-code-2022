use itertools::Itertools;
use reikna::factor::lcm_all;
use std::collections::VecDeque;
use std::io::BufRead;

#[derive(Debug)]
enum Op {
    Add(u64),
    AddO,
    Multi(u64),
    MultiO,
}

impl Op {
    fn apply(&self, o: u64) -> u64 {
        match self {
            Op::Add(n) => o + n,
            Op::AddO => o + o,
            Op::Multi(n) => o * n,
            Op::MultiO => o * o,
        }
    }

    fn apply_lcm(&self, o: u64, lcm: u64) -> u64 {
        match self {
            Op::Add(n) => o + n,
            Op::AddO => o + o,
            Op::Multi(n) => (o * n) % lcm,
            Op::MultiO => (o * o) % lcm,
        }
    }
}

#[derive(Debug)]
struct Next {
    div: u64,
    t: usize,
    f: usize,
}
impl Next {
    fn apply(&self, o: u64) -> usize {
        if o % self.div == 0 {
            self.t
        } else {
            self.f
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    next_monkey: Next,
    is: u64, // inspections
}

impl Monkey {
    fn add_inspection(&mut self) {
        self.is += 1;
    }
}

pub fn part1() {
    let mut input = get_input();
    let result = solve(&mut input, 20, None);
    println!("D11P1 {}", result);
}

pub fn part2() {
    let mut input = get_input();
    let n: &[u64] = &input
        .iter()
        .map(|m| m.next_monkey.div)
        .collect::<Vec<u64>>();
    let lcm = lcm_all(n);
    let result = solve(&mut input, 10_000, Some(lcm));
    println!("D11P2 {}", result);
}

fn solve(input: &mut Vec<Monkey>, rounds: u32, lcm: Option<u64>) -> u64 {
    let m_max = input.len();
    for _ in 0..rounds {
        for mi in 0..m_max {
            while let Some(item) = &input[mi].items.pop_front() {
                let _ = &input[mi].add_inspection();
                let worry_level = {
                    if let Some(lcm) = lcm {
                        *(&input[mi].op.apply_lcm(*item, lcm))
                    } else {
                        *(&input[mi].op.apply(*item)) / 3
                    }
                };
                let next = &input[mi].next_monkey.apply(worry_level);
                let _ = &input[*next].items.push_back(worry_level);
            }
        }
    }

    let sorted: Vec<&Monkey> = input.iter().sorted_by_key(|m| m.is).collect();
    let result = sorted[m_max - 1].is * sorted[m_max - 2].is;
    result
}

fn get_input() -> Vec<Monkey> {
    include_bytes!("../inputs/day11.txt")
        .lines()
        .filter_map(|l| l.ok())
        .chunks(7)
        .into_iter()
        .map(|monkey| {
            let monkey_: Vec<String> = monkey.collect();
            let items: VecDeque<u64> = {
                String::from(&monkey_[1][18..])
                    .split(", ")
                    .filter_map(|i| i.parse().ok())
                    .collect()
            };
            let op = {
                let rest = String::from(&monkey_[2][23..]);
                let symbol = rest.as_bytes()[0];
                match String::from(&rest[2..]).parse() {
                    Ok(t) => match symbol {
                        b'*' => Op::Multi(t),
                        b'+' => Op::Add(t),
                        _ => panic!("Invalid op"),
                    },
                    Err(_) => match symbol {
                        b'*' => Op::MultiO,
                        b'+' => Op::AddO,
                        _ => panic!("Invalid op"),
                    },
                }
            };
            let next_monkey = {
                let div: u64 = String::from(&monkey_[3][21..]).parse().unwrap();
                let t: usize = String::from(&monkey_[4][29..]).parse().unwrap();
                let f: usize = String::from(&monkey_[5][30..]).parse().unwrap();
                Next { div, t, f }
            };
            Monkey {
                items,
                op,
                next_monkey,
                is: 0,
            }
        })
        .collect()
}
