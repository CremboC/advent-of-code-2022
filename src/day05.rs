use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::sequence::tuple;
use std::collections::VecDeque;
use std::fs;

struct Move {
    count: u8,
    from: usize,
    to: usize,
}

fn parse_movement(input: &str) -> nom::IResult<&str, Move> {
    let parser = tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ));
    map(
        parser,
        |(_, count, _, from, _, to): (_, &str, _, &str, _, &str)| Move {
            count: count.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        },
    )(input)
}

fn parse_input() -> (Vec<Move>, Vec<VecDeque<u8>>) {
    let input = fs::read_to_string("inputs/day05.txt").expect("Failed to read file.");
    let inp = input.split("\n\n").collect::<Vec<&str>>();
    let towers_unparsed = {
        let mut iter = inp.first().unwrap().split('\n').into_iter();

        iter.next_back();
        iter.map(|line| {
            line.replace("] [", " ")
                .replace("[", "")
                .replace("]", "")
                .replace("    ", "_")
                .replace(" ", "")
        })
    }
    .collect::<Vec<String>>();

    let tower_count = towers_unparsed
        .iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .len();

    let mut towers = Vec::with_capacity(tower_count);
    for i in 0..tower_count {
        let mut tower = VecDeque::new();
        towers_unparsed
            .iter()
            .for_each(|line| match line.as_bytes().get(i) {
                Some(&l) if l != b'_' => tower.push_back(l),
                _ => {}
            });
        towers.push(tower)
    }

    let movements = inp
        .last()
        .unwrap()
        .split('\n')
        .map(|line| {
            let (_, movement) = parse_movement(line).expect("Invalid movement line");
            movement
        })
        .collect::<Vec<Move>>();

    (movements, towers)
}

pub fn part1() {
    let (movements, mut towers) = parse_input();

    for x in movements {
        for _ in 0..x.count {
            let popped = towers[x.from].pop_front().unwrap();
            towers[x.to].push_front(popped);
        }
    }

    let out = towers
        .iter()
        .filter_map(|t| t.front().map(|&x| x as char))
        .collect::<String>();

    println!("D05P1 {:?}", out);
}

pub fn part2() {
    let (movements, mut towers) = parse_input();

    for x in movements {
        let mut popped = (0..x.count)
            .filter_map(|_| towers[x.from].pop_front())
            .collect::<Vec<u8>>();

        popped.reverse();

        for y in popped {
            towers[x.to].push_front(y);
        }
    }

    let out = towers
        .iter()
        .filter_map(|t| t.front().map(|&x| x as char))
        .collect::<String>();

    println!("D05P2 {:?}", out);
}
