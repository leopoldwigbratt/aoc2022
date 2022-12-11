use std::error::Error;

mod utils;
mod day1;   mod day9;
mod day2;   mod day10;
mod day3;   mod day11;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

type SolverResult = Result<(String, String), Box<dyn Error>>;
type Solver = fn() -> SolverResult;

const DAYS: usize = 11;
const SOLVERS: [Solver; DAYS] = [
    day1::solve,    day2::solve,    day3::solve,    day4::solve,
    day5::solve,    day6::solve,    day7::solve,    day8::solve,
    day9::solve,    day10::solve,   day11::solve,
];

fn main() {
    for i in 0..DAYS {
        match SOLVERS[i]() {
            Ok((part1, part2)) => println!("\x1b[1mDay {:2}\x1b[0m \x1b[94mPart 1\x1b[0m {part1:10} \x1b[93mPart 2\x1b[0m {part2}", i + 1),
            Err(error) => eprintln!("\x1b[91mError on Day {:2}\x1b[0m: {error}", i + 1),
        }
    }
}
