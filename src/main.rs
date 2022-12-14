mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

use std::time::Instant;

fn main() {
    time("day 1 part 1", || day01::part1());
    time("day 1 part 2", || day01::part2());

    time("day 2 part 1", || day02::part1());
    time("day 2 part 2", || day02::part2());

    time("day 3 part 1", || day03::part1());
    time("day 3 part 2", || day03::part2());

    time("day 4 part 1", || day04::part1());
    time("day 4 part 2", || day04::part2());

    time("day 5 part 1", || day05::part1());
    time("day 5 part 2", || day05::part2());

    time("day 6 part 1", || day06::part1());
    time("day 6 part 2", || day06::part2());

    time("day 7 part 1", || day07::part1());
    time("day 7 part 2", || day07::part2());

    time("day 8 part 1", || day08::part1());
    time("day 8 part 2", || day08::part2());

    time("day 9 part 1", || day09::part1());
    time("day 9 part 2", || day09::part2());

    time("day 10 part 1", || day10::part1());
    time("day 10 part 2", || day10::part2());

    time("day 11 part 1", || day11::part1());
    time("day 11 part 2", || day11::part2());

    time("day 12 part 1", || day12::part1());
    time("day 12 part 2", || day12::part2());

    time("day 13 part 1", || day13::part1());
    time("day 13 part 2", || day13::part2());

    time("day 14 part 1", || day14::part1());
    time("day 14 part 2", || day14::part2());

    time("day 15 part 1", || day15::part1());
    time("day 15 part 2", || day15::part2());
}

fn time(name: &str, f: fn()) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("Time elapsed in {:?} is: {:?}\n", name, duration);
}
