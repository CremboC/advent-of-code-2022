use std::fs;

pub fn part1() {
    struct State {
        current_sum: i32,
        max: i32,
    }

    impl State {
        fn update_current(&self, increment: i32) -> State {
            State {
                current_sum: self.current_sum + increment,
                max: self.max,
            }
        }

        fn maybe_update_max(&self) -> State {
            return if self.current_sum > self.max {
                State {
                    current_sum: 0,
                    max: self.current_sum,
                }
            } else {
                State {
                    current_sum: 0,
                    max: self.max,
                }
            };
        }
    }

    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read file.");

    let state = State {
        current_sum: 0,
        max: i32::MIN,
    };
    let result = input.split("\n").fold(state, |acc, line| {
        return if line.is_empty() {
            acc.maybe_update_max()
        } else {
            let number = line.parse::<i32>().expect("Expected integer");
            acc.update_current(number)
        };
    });

    let max = result.max;
    println!("Max is {max}");
}

pub fn part2() {
    struct State {
        current_sum: i32,
        max: [i32; 3],
    }

    impl State {
        fn update_current(&mut self, increment: i32) {
            self.current_sum += increment;
        }

        fn maybe_update_max(&mut self) {
            match self.max.iter().enumerate().min_by_key(|(_idx, el)| **el) {
                None => {}
                Some((idx, &el)) => {
                    if self.current_sum > el {
                        self.max[idx] = self.current_sum;
                    }
                }
            }

            self.current_sum = 0;
        }
    }

    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read file.");

    let state = State {
        current_sum: 0,
        max: [0, 0, 0],
    };

    let result = input.split("\n").fold(state, |mut acc, line| {
        if line.is_empty() {
            acc.maybe_update_max()
        } else {
            let number = line.parse::<i32>().expect("Expected integer");
            acc.update_current(number)
        }
        return acc;
    });

    let sum = result.max.iter().sum::<i32>();
    println!("Sum is {sum}");
}
