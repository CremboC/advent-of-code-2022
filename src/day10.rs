use itertools::Itertools;
use std::collections::VecDeque;
use std::io::BufRead;

#[derive(Debug)]
enum I {
    Noop,
    Addx(i32),
}

pub fn part1() {
    let (result, _) = solve();
    println!("D10P1 {}", result);
}

pub fn part2() {
    let (_, screen) = solve();
    println!("D10P2");
    print_screen(&screen);
}

fn solve() -> (i32, [char; 240]) {
    let input = get_input();
    let mut result = 0_i32;
    let mut screen = [' '; 240];

    let pl = input.len(); // program length
    let mut rx = 1_i32; // register X
    let mut cycle = 1_i32; // cycle count
    let mut pc = 0; // program pointer
    let cycle_checks: Vec<i32> = (20..=220).step_by(40).collect();

    let mut exec: VecDeque<&I> = VecDeque::new();
    loop {
        if cycle_checks.contains(&cycle) {
            result += cycle as i32 * rx;
        }

        let exec_next = exec.pop_front();
        let cycle_index = {
            let rem = cycle % 40;
            if rem == 0 {
                40
            } else {
                rem
            }
        };

        if cycle_index == rx || cycle_index == rx + 1 || cycle_index == rx + 2 {
            screen[(cycle - 1) as usize] = '█';
        }

        if let Some(i) = exec_next {
            match i {
                I::Noop => {}
                I::Addx(v) => {
                    rx += v;
                }
            }
        }

        if pc < pl {
            let instr = &input[pc];

            match instr {
                I::Noop => {
                    if exec_next.is_some() {
                        exec.push_back(instr)
                    }
                }
                I::Addx(_) => {
                    if exec_next.is_some() {
                        exec.push_back(&I::Noop)
                    }
                    exec.push_back(instr);
                }
            }

            pc += 1;
        }

        if exec.is_empty() && pc == pl {
            break;
        }
        cycle += 1;
    }

    return (result, screen);
}

fn print_screen(screen: &[char; 240]) {
    screen.chunks(40).for_each(|line| {
        println!("{}", line.iter().join(""));
    });
}

fn get_input() -> Vec<I> {
    include_bytes!("../inputs/day10.txt")
        .lines()
        .map(|line| {
            let string = line.unwrap();
            if string == "noop" {
                I::Noop
            } else {
                let (_, n) = string.split(' ').collect_tuple::<(&str, &str)>().unwrap();
                I::Addx(n.parse().unwrap())
            }
        })
        .collect()
}
