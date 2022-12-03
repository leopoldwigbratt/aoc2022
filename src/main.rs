use std::error::Error;

mod utils;
mod day1;
mod day2;

type Solver = fn() -> Result<(String, String), Box<dyn Error>>;

const DAYS: usize = 2;
const SOLVERS: [Solver; DAYS] = [day1::solve, day2::solve];

fn main() {
    for i in 0..DAYS {
        match SOLVERS[i]() {
            Ok((part1, part2)) => println!("Day {}, part 1: {}, part 2: {}", i + 1, part1, part2),
            Err(error) => eprintln!("Error on Day {}: {}", i + 1, error),
        }
    }
}
