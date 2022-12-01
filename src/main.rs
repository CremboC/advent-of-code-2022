mod day01;

use std::time::Instant;

fn main() {
    time(String::from("part1"), || day01::part1());
    time(String::from("part2"), || day01::part2());
}

fn time(name: String, f: fn()) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("Time elapsed in {:?} is: {:?}", name, duration);
}
