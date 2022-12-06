use std::error::Error;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

type SolverResult = Result<(String, String), Box<dyn Error>>;
type Solver = fn() -> SolverResult;

const DAYS: usize = 6;
const SOLVERS: [Solver; DAYS] = [day1::solve, day2::solve, day3::solve, day4::solve, day5::solve, day6::solve];

fn main() {
    for i in 0..DAYS {
        match SOLVERS[i]() {
            Ok((part1, part2)) => println!("Day {}, part 1: {}, part 2: {}", i + 1, part1, part2),
            Err(error) => eprintln!("Error on Day {}: {}", i + 1, error),
        }
    }
}
