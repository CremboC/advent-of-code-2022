use itertools::Itertools;
use ndarray::Array2;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use num_traits::identities::Zero;
use std::io::BufRead;
use std::ops::Add;

pub fn part1() {
    let input = get_input();
    let (max_y, max_x) = find_min_max(&input);
    let mut cave = Array2::<E>::zeros((max_y, max_x));
    add_blockers(&input, &mut cave);

    let start = Pos::new(0, 500);
    for round in 0.. {
        if let Some(empty) = find_first_empty(&cave, &start, max_y) {
            cave[(empty.y, empty.x)] = E::Sand;
        } else {
            println!("D14P1 {}", round);
            break;
        }
    }
}

pub fn part2() {
    let mut input = get_input();
    let (max_y, max_x_calc) = find_min_max(&input);
    let floor = max_y + 1;
    let max_x = max_x_calc * 2;
    let mut cave = Array2::<E>::zeros((floor + 2, max_x + 1));

    input.push(vec![Pos::new(floor, 0), Pos::new(floor, max_x)]);
    add_blockers(&input, &mut cave);
    let start = Pos::new(0, 500);
    for round in 0.. {
        if let Some(empty) = find_first_empty_p2(&cave, &start, &start) {
            cave[(empty.y, empty.x)] = E::Sand;
        } else {
            println!("D14P2 {}", round + 1);
            break;
        }
    }
}

fn find_min_max(input: &Vec<Vec<Pos>>) -> (usize, usize) {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    for p in input.iter().flat_map(|v| v.iter()) {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    max_x += 1;
    max_y += 1;
    (max_y, max_x)
}

fn find_first_empty(cave: &Array2<E>, start: &Pos, max_y: usize) -> Option<Pos> {
    let x = start.x;
    for y in start.y.. {
        if y > max_y {
            println!("Reached limit of y={}/{}", y, max_y);
            return None;
        }
        if let Some(e) = cave.get((y, x)) {
            if !e.is_free() {
                let diag_left = cave.get((y, x - 1)).unwrap();
                let diag_right = cave.get((y, x + 1)).unwrap();
                return if diag_left.is_free() {
                    let start_n = Pos::new(y, x - 1);
                    find_first_empty(cave, &start_n, max_y)
                } else if diag_right.is_free() {
                    let start_n = Pos::new(y, x + 1);
                    find_first_empty(cave, &start_n, max_y)
                } else {
                    Some(Pos::new(y - 1, x))
                };
            }
        }
    }

    None
}

fn find_first_empty_p2(cave: &Array2<E>, start: &Pos, end: &Pos) -> Option<Pos> {
    let x = start.x;
    for y in start.y.. {
        if let Some(e) = cave.get((y, x)) {
            if !e.is_free() {
                let diag_left = cave.get((y, x - 1)).unwrap();
                let diag_right = cave.get((y, x + 1)).unwrap();
                return if diag_left.is_free() {
                    let start_n = Pos::new(y, x - 1);
                    find_first_empty_p2(cave, &start_n, end)
                } else if diag_right.is_free() {
                    let start_n = Pos::new(y, x + 1);
                    find_first_empty_p2(cave, &start_n, end)
                } else {
                    let pos = Pos::new(y - 1, x);
                    if pos == *end {
                        println!("Found end");
                        None
                    } else {
                        Some(pos)
                    }
                };
            }
        }
    }
    None
}

fn add_blockers(input: &Vec<Vec<Pos>>, cave: &mut Array2<E>) {
    for scan in input {
        for (from, to) in scan.iter().tuple_windows::<(_, _)>() {
            if from.x == to.x {
                for y in from.y.min(to.y)..=to.y.max(from.y) {
                    cave[(y, from.x)] = E::Rock;
                }
            } else {
                for x in from.x.min(to.x)..=to.x.max(from.x) {
                    cave[(from.y, x)] = E::Rock;
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum E {
    Rock,
    Sand,
    Air,
}

impl E {
    fn is_free(&self) -> bool {
        match self {
            E::Rock => false,
            E::Sand => false,
            E::Air => true,
        }
    }
}

impl Add<Self> for E {
    type Output = E;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Zero for E {
    fn zero() -> Self {
        E::Air
    }

    fn is_zero(&self) -> bool {
        *self == E::Air
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pos {
    y: usize,
    x: usize,
}
impl Pos {
    fn new(y: usize, x: usize) -> Pos {
        Pos { y, x }
    }
}

fn get_input() -> Vec<Vec<Pos>> {
    include_bytes!("../inputs/day14.txt")
        .lines()
        .map(|l| parse(l.unwrap().as_str()).unwrap().1)
        .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<Pos>> {
    separated_list1(
        tag(" -> "),
        map(
            separated_pair(digit1, tag(","), digit1),
            |coord: (&str, &str)| {
                let (x, y) = coord;
                Pos::new(y.parse().unwrap(), x.parse().unwrap())
            },
        ),
    )(input)
}
