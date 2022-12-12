use std::{error::Error, time::Instant};

mod utils;
mod day1;   mod day9;
mod day2;   mod day10;
mod day3;   mod day11;
mod day4;   mod day12;
mod day5;
mod day6;
mod day7;
mod day8;

type SolverResult = Result<(String, String), Box<dyn Error>>;
type Solver = fn() -> SolverResult;

const DAYS: usize = 12;
const SOLVERS: [Solver; DAYS] = [
    day1::solve,    day2::solve,    day3::solve,    day4::solve,
    day5::solve,    day6::solve,    day7::solve,    day8::solve,
    day9::solve,    day10::solve,   day11::solve,   day12::solve,
];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let Some(command) = args.get(1) else {
        eprintln!("A command must be specified. Available: day, all.");
        return;
    };

    match command.as_str() {
        "all" => run_all(),
        "day" => {
            let Some(day) = args.get(2) else {
                eprintln!("A day must be specified.");
                return;
            };

            let Ok(day) = day.parse::<usize>() else {
                eprintln!("An integer between 1 and {DAYS} must be specified (received {day})");
                return;
            };

            if day > DAYS {
                eprintln!("An integer between 1 and {DAYS} must be specified (received {day})");
                return;
            }

            run_day(day);
        },
        rec => eprintln!("{rec} is not a valid command. Available commans are: day, all."),
    }
}

fn run_all() {
    let start_total = Instant::now();

    println!("\x1b[1mDay\x1b[0m | \x1b[94mPart 1\x1b[0m       | \x1b[93mPart2\x1b[0m        | Time (ms)");
    
    for i in 0..DAYS {
        let start_solution = Instant::now();
        let solution = SOLVERS[i]();
        let elapsed_solution = start_solution.elapsed().as_micros();
        let time = (elapsed_solution as f64) / 1000.0;
        match solution {
            Ok((part1, part2)) => println!("\x1b[1m{:<2}\x1b[0m  | {part1:12} | {part2:12} | {time:07.3}", i + 1),
            Err(error) => eprintln!("\x1b[91mError on Day {:2}\x1b[0m: {error}", i + 1),
        }
    }

    let elapsed_total = start_total.elapsed().as_millis();

    println!("\nTotal Time Elapsed: {elapsed_total} ms");
}

fn run_day(day: usize) {
    let start = Instant::now();
    
    println!("Running \x1b[1mDay {day}\x1b[0m");
    
    println!("\x1b[94mPart 1\x1b[0m       | \x1b[93mPart2\x1b[0m        | Time (ms)");

    let solution = SOLVERS[day - 1]();

    let elapsed = start.elapsed().as_micros();
    let time = (elapsed as f64) / 1000.0;
    
    match solution {
        Ok((part1, part2)) => println!("{part1:12} | {part2:12} | {time:07.3}"),
        Err(error) => eprintln!("\x1b[91mError on Day {day:2}\x1b[0m: {error}"),
    }
}