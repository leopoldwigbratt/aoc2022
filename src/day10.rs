use lazy_static::lazy_static;
use regex::Regex;

use std::error::Error;

use crate::utils::read_input_lines_ok;

#[derive(PartialEq, Debug)]
enum Instruction {
  Noop,
  Addx(i32),
}

const REGEX_PATTERN: &str = r"^(?P<noop>noop)|(?P<addx>addx (?P<value>-?\d+))$";

lazy_static! {
  static ref REGEX: Regex = Regex::new(REGEX_PATTERN).unwrap();
}

const LIT: char = '#';
const DARK: char = '.';

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines = read_input_lines_ok("day10.txt")?;

  let (_, _, part1) = lines
    .iter()
    .map(parse_line)
    .fold((0, 1, 0), step_instruction);

  let (_, _, part2) = lines
    .iter()
    .map(parse_line)
    .fold((0, 1, String::with_capacity(240)), draw_cycle);
  
  Ok((part1.to_string(), part2))
}

fn draw_cycle((mut cycle, x_reg, mut output): (i32, i32, String), instruction: Instruction) -> (i32, i32, String) {
  match instruction {
    Instruction::Noop => {
      draw(cycle, x_reg, &mut output);
      (cycle + 1, x_reg, output)
    },
    Instruction::Addx(value) => {
      draw(cycle, x_reg, &mut output);
      cycle += 1;
      draw(cycle, x_reg, &mut output);
      cycle += 1;
      (cycle, x_reg + value, output)
    }
  }
}

fn draw(cycle: i32, x_reg: i32, output: &mut String) {
  if cycle % 40 == 0 {
    output.push('\n');
  }

  let offset = (cycle / 40) * 40;
  if cycle >= x_reg - 1 + offset && cycle <= x_reg + 1 + offset {
    output.push(LIT);
  } else {
    output.push(DARK);
  }
}

fn step_instruction((mut cycle, x_reg, mut sum): (i32, i32, i32), instr: Instruction) -> (i32, i32, i32) {
  match instr {
    Instruction::Noop => {
      (cycle + 1, x_reg, get_sum(cycle + 1, x_reg, sum))
    },
    Instruction::Addx(value) => {
      cycle += 1;
      sum = get_sum(cycle, x_reg, sum);
      cycle += 1;
      sum = get_sum(cycle, x_reg, sum);
      (cycle, x_reg + value, sum)
    },
  }
}

fn get_sum(cycle: i32, x_reg: i32, sum: i32) -> i32 {
  if (cycle - 20) % 40 == 0 {
    sum + cycle * x_reg
  } else {
    sum
  }
}

fn parse_line(line: &String) -> Instruction {
  let captures = REGEX.captures(line).unwrap();

  if let Some(_) = captures.name("noop") {
    return Instruction::Noop;
  }

  if let Some(_) = captures.name("addx") {
    let value = captures.name("value").unwrap().as_str().parse().unwrap();
    return Instruction::Addx(value);
  }

  panic!("Failed to parse line, received: {}", line);
}

#[cfg(test)]
mod tests {
  use super::{Instruction, parse_line, get_sum, step_instruction};

  use Instruction::*;

  #[test]
  fn test_step_instruction() {
    assert_eq!(step_instruction((0, 0, 0), Noop), (1, 0, 0));
    assert_eq!(step_instruction((1, 0, 0), Addx(3)), (3, 3, 0));
    assert_eq!(step_instruction((3, 3, 0), Addx(-5)), (5, -2, 0));

    assert_eq!(step_instruction((19, 21, 0), Addx(-1)), (21, 20, 420));
    assert_eq!(step_instruction((59, 19, 0), Noop), (60, 19, 1140));

    assert_eq!(vec![Noop, Addx(3), Addx(-5)].into_iter().fold((0, 1, 0), step_instruction), (5, -1, 0));
  }

  #[test]
  fn test_get_sum() {
    assert_eq!(get_sum(0, 0, 0), 0);
    assert_eq!(get_sum(32, 23, 0), 0);
    assert_eq!(get_sum(52, 35, 65), 65);

    assert_eq!(get_sum(20, 21, 0), 420);
    assert_eq!(get_sum(60, 19, 0), 1140);
    assert_eq!(get_sum(100, 18, 0), 1800);
    assert_eq!(get_sum(140, 21, 0), 2940);
    assert_eq!(get_sum(180, 16, 0), 2880);
    assert_eq!(get_sum(220, 18, 0), 3960);
  }
  
  #[test]
  fn test_parse_line() {
    assert_eq!(parse_line(&String::from("noop")), Noop);
    (-101..101).for_each(|value| {
      assert_eq!(parse_line(&format!("addx {}", value)), Addx(value));
    });
  }
}