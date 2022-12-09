use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part1() {
    const X_MAX: usize = 99;
    const Y_MAX: usize = 99;

    let grid = include_bytes!("../inputs/day08.txt")
        .split(|&x| x == b'\n')
        .map(|line| line.iter().map(|&s| s).collect())
        .collect::<Vec<Vec<u8>>>();

    let mut visible = 0_u32;
    for y in 1..Y_MAX - 1 {
        for x in 1..X_MAX - 1 {
            let current_tree = &grid[y][x];
            let is_visible = grid[y][0..x].iter().all(|i| i < current_tree)
                || grid[y][(x + 1)..X_MAX].iter().all(|i| i < current_tree)
                || grid[0..y].iter().map(|l| &l[x]).all(|i| i < current_tree)
                || grid[(y + 1)..Y_MAX]
                    .iter()
                    .map(|l| &l[x])
                    .all(|i| i < current_tree);

            if is_visible {
                visible += 1
            }
        }
    }
    println!(
        "D08P1 {}",
        visible + (X_MAX as u32 * 2) + (Y_MAX as u32 * 2) - 4
    );
}

pub fn part2() {
    const X_MAX: usize = 99;
    const Y_MAX: usize = 99;

    let grid = include_bytes!("../inputs/day08.txt")
        .split(|&x| x == b'\n')
        .map(|line| line.iter().map(|&s| s).collect())
        .collect::<Vec<Vec<u8>>>();

    let mut best = u32::MIN;
    for y in 1..Y_MAX - 1 {
        for x in 1..X_MAX - 1 {
            let current_tree = grid[y][x];

            let score_west = grid[y][0..x]
                .iter()
                .rev()
                .fold_while(0, |acc, &x| {
                    if x < current_tree {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let score_north = grid[0..y]
                .iter()
                .map(|l| &l[x])
                .rev()
                .fold_while(0, |acc, &x| {
                    if x < current_tree {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let score_east = grid[y][(x + 1)..X_MAX]
                .iter()
                .fold_while(0, |acc, &x| {
                    if x < current_tree {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let score_south: u32 = grid[(y + 1)..Y_MAX]
                .iter()
                .map(|l| &l[x])
                .fold_while(0, |acc, &x| {
                    if x < current_tree {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let score = score_west * score_north * score_east * score_south;
            if score > best {
                best = score;
            }
        }
    }
    println!("D08P2 {}", best);
}
