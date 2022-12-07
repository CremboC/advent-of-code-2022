mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use std::time::Instant;

fn main() {
    time(String::from("day 1 part 1"), || day01::part1());
    time(String::from("day 1 part 2"), || day01::part2());

    time(String::from("day 2 part 1"), || day02::part1());
    time(String::from("day 2 part 2"), || day02::part2());

    time(String::from("day 3 part 1"), || day03::part1());
    time(String::from("day 3 part 2"), || day03::part2());

    time(String::from("day 4 part 1"), || day04::part1());
    time(String::from("day 4 part 2"), || day04::part2());

    time(String::from("day 5 part 1"), || day05::part1());
    time(String::from("day 5 part 2"), || day05::part2());

    time(String::from("day 6 part 1"), || day06::part1());
    time(String::from("day 6 part 2"), || day06::part2());

    time(String::from("day 7 part 1"), || day07::part1());
    time(String::from("day 7 part 2"), || day07::part2());
}

fn time(name: String, f: fn()) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("Time elapsed in {:?} is: {:?}\n", name, duration);
}
