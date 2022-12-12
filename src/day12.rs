use ndarray::Array2;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn part1() {
    let (input, start, end) = get_input();
    println!("D12P1 {:?}", run_astar_cost(&input, start, end).unwrap());
}

pub fn part2() {
    let (input, _, end) = get_input();

    let starts: Vec<Pos> = input
        .indexed_iter()
        .filter_map(|((y, x), &c)| {
            if c == b'a' {
                Some((y as i32, x as i32))
            } else {
                None
            }
        })
        .collect();

    let best_cost = starts
        .iter()
        .filter_map(|&start| run_astar_cost(&input, start, end))
        .min()
        .unwrap_or(u64::MAX);

    println!("D12P2 {:?}", best_cost);
}

fn run_astar_cost(input: &Array2<u8>, start: Pos, end: Pos) -> Option<u64> {
    astar_cost(
        start,
        end,
        |f| dist(f, end),
        |(y, x)| {
            let current = *input.get((y as usize, x as usize)).unwrap();
            let to_check = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

            to_check
                .iter()
                .filter_map(|(y, x)| {
                    input.get((*y as usize, *x as usize)).and_then(|&target| {
                        if target > current && target - current == 1 {
                            Some(((*y, *x), 1))
                        } else if target == current || target < current {
                            Some(((*y, *x), (current - target).max(1)))
                        } else {
                            None
                        }
                    })
                })
                .collect()
        },
    )
}

fn astar_cost<Hf, Nf>(
    start: Pos,
    goal: Pos,
    get_h: Hf, // heuristic cost
    get_n: Nf, // neighbours
) -> Option<u64>
where
    Hf: Fn(Pos) -> u64,
    Nf: Fn(Pos) -> Vec<(Pos, u8)>,
{
    let mut open_queue = PriorityQueue::new();
    open_queue.push(start, Reverse(0));

    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, u64> = HashMap::new();
    g_score.insert(start, 0);

    while let Some((current, _)) = open_queue.pop() {
        if current == goal {
            return Some(path_length(&came_from, current));
        }

        let neighbours = get_n(current);
        for (n, cost) in neighbours {
            let tentative_g_score = g_score
                .entry(current)
                .or_insert(u64::MAX)
                .saturating_add(cost as u64);
            let n_g_score = *g_score.entry(n).or_insert(u64::MAX);
            if tentative_g_score < n_g_score {
                came_from.insert(n, current);
                g_score.insert(n, tentative_g_score);
                let cost_new = tentative_g_score + get_h(n);
                open_queue.push(n, Reverse(cost_new));
            }
        }
    }

    None
}

fn path_length(came_from: &HashMap<Pos, Pos>, current: Pos) -> u64 {
    let mut curr = current;
    let mut len = 0_u64;
    while let Some(&p) = came_from.get(&curr) {
        curr = p;
        len += 1;
    }
    len
}

fn dist(from: Pos, to: Pos) -> u64 {
    ((from.0.abs_diff(to.0)) + (from.1.abs_diff(to.1))) as u64
}

fn find(input: &Array2<u8>, target: u8) -> Option<Pos> {
    input
        .indexed_iter()
        .find(|(_, &el)| el == target)
        .map(|((y, x), _)| (y as i32, x as i32))
}

type Pos = (i32, i32);

fn get_input() -> (Array2<u8>, Pos, Pos) {
    let input = include_bytes!("../inputs/day12.txt");
    let (max_y, max_x) = {
        let max_x = input.iter().take_while(|&&c| c != b'\n').count();
        let max_y = input.iter().filter(|&&c| c == b'\n').count();
        (max_y + 1, max_x)
    };

    let mut arr = Array2::from_shape_vec(
        (max_y, max_x),
        input
            .iter()
            .filter_map(|&c| if c == b'\n' { None } else { Some(c) })
            .collect(),
    )
    .expect("Matrix to be valid");

    let start = find(&arr, b'S').expect("Start must exist");
    let end = find(&arr, b'E').expect("End must exist");
    let start_mut = arr.get_mut((start.0 as usize, start.1 as usize)).unwrap();
    *start_mut = b'a';
    let end_mut = arr.get_mut((end.0 as usize, end.1 as usize)).unwrap();
    *end_mut = b'z';

    (arr, start, end)
}
