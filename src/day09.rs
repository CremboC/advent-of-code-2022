use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
struct Pos {
    y: i32,
    x: i32,
}
impl Pos {
    fn new(y: i32, x: i32) -> Pos {
        Pos { y, x }
    }

    fn add(&mut self, add: (i32, i32)) {
        self.x += add.1;
        self.y += add.0;
    }
}

pub fn part1() {
    let input = get_input();
    let mut visited_set = HashSet::<(i32, i32)>::new();
    let mut head_pos = Pos::new(0, 0);
    let mut tail_pos = Pos::new(0, 0);
    visited_set.insert((tail_pos.y, tail_pos.x));

    for (add, m) in input {
        for _ in 0..m {
            let head_prev_x = head_pos.x;
            let head_prev_y = head_pos.y;
            head_pos.add(add);
            if !are_adjacent(&head_pos, &tail_pos) {
                tail_pos.x = head_prev_x;
                tail_pos.y = head_prev_y;
                visited_set.insert((tail_pos.y, tail_pos.x));
            }
        }
    }

    let result = visited_set.len();
    println!("D09P1 {}", result);
}

pub fn part2() {
    let input = get_input();
    let mut visited_set = HashSet::<(i32, i32)>::new();
    let mut rope = [Pos::new(0, 0); 10];
    visited_set.insert((0, 0));
    for (add, m) in input {
        for _ in 0..m {
            rope[0].add(add);
            for (i, j) in (0..10).tuple_windows::<(_, _)>() {
                if !are_adjacent(&rope[i], &rope[j]) {
                    rope[j].add(get_add(&rope[i], &rope[j]));
                }
            }
            visited_set.insert((rope[9].y, rope[9].x));
        }
    }

    let result = visited_set.len();
    println!("D09P1 {}", result);
}

fn are_adjacent(p1: &Pos, p2: &Pos) -> bool {
    (p1.x.abs_diff(p2.x) <= 1) && (p1.y.abs_diff(p2.y) <= 1)
}

fn get_add(h: &Pos, t: &Pos) -> (i32, i32) {
    if t.x == h.x {
        let yd = if h.y > t.y { -1 } else { 1 };
        (h.y - t.y + yd, 0)
    } else if t.y == h.y {
        let xd = if h.x > t.x { -1 } else { 1 };
        (0, h.x - t.x + xd)
    } else {
        let x_o = if h.x > t.x { 1 } else { -1 };
        let y_o = if h.y > t.y { 1 } else { -1 };
        (y_o, x_o)
    }
}

fn get_input() -> Vec<((i32, i32), u8)> {
    include_bytes!("../inputs/day09.txt")
        .lines()
        .map(|line| {
            let string = line.unwrap();
            let (dir, n) = string.split(' ').collect_tuple::<(&str, &str)>().unwrap();
            let m: u8 = n.parse().unwrap();
            match dir {
                "R" => ((0, 1), m),
                "D" => ((1, 0), m),
                "L" => ((0, -1), m),
                "U" => ((-1, 0), m),
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}
