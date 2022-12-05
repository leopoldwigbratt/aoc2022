use regex::Regex;

use std::error::Error;
use std::collections::VecDeque;

use crate::utils::read_input_lines_ok;

static REGEX_STACKS: &str = r"((\[(?P<letter>[A-Z])\]|(?P<space> {3})) ?)";
static REGEX_TERMINATE: &str = r"^( [0-9]+ )( ( [0-9]+ ))*$";
static REGEX_INSTRUCTION: &str = r"^move (?P<move>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)$";

#[derive(Debug, PartialEq)]
struct Instruction(u8, u8, u8);

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines = read_input_lines_ok("day5.txt")?;

  let (mut stacks, index) = parse_stacks(&lines);

  let instructions = parse_instructions(&lines[(index as usize)..]);

  let mut stacks2 = stacks.clone();

  instructions.iter().for_each(|instruction| {
    execute_instruction(&mut stacks, instruction);
    execute_instruction2(&mut stacks2, instruction)
  });

  Ok((get_top_of_stacks(&stacks), get_top_of_stacks(&stacks2)))
}

fn get_top_of_stacks(stacks: &Vec<VecDeque<u8>>) -> String {
  stacks.iter().fold(String::new(), |mut string, stack| {
    let last = stack.len() - 1;
    string.push(*stack.get(last).unwrap() as char);
    string
  })
}

fn execute_instruction2(stacks: &mut Vec<VecDeque<u8>>, Instruction(m, f, t): &Instruction) {
  let stack_from = stacks.get_mut(*f as usize - 1).unwrap();
  let items = stack_from.split_off(stack_from.len() - *m as usize);
  let stack_to = stacks.get_mut(*t as usize - 1).unwrap();
  items.into_iter().for_each(|item| stack_to.push_back(item));
}

fn execute_instruction(stacks: &mut Vec<VecDeque<u8>>, Instruction(m, f, t): &Instruction) {
  (0..*m).for_each(|_| {
    let stack_from = stacks.get_mut(*f as usize - 1).unwrap();
    let item = stack_from.pop_back().unwrap();
    let stack_to = stacks.get_mut(*t as usize - 1).unwrap();
    stack_to.push_back(item);
  });
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
  let regex = Regex::new(REGEX_INSTRUCTION).unwrap();
  lines.iter().fold(Vec::new(), |mut vec, line| {
    let captures = regex.captures(line).unwrap();
    vec.push(
      Instruction(
        captures.name("move").unwrap().as_str().parse::<u8>().unwrap(),
        captures.name("from").unwrap().as_str().parse::<u8>().unwrap(),
        captures.name("to").unwrap().as_str().parse::<u8>().unwrap()
      )
    );
    vec
  })
}

fn init_stacks(line: &str) -> Vec<VecDeque<u8>> {
  let regex = Regex::new(REGEX_STACKS).unwrap();
  regex.captures_iter(line).fold(Vec::new(), |mut stacks, cap| {
    if let Some(letter) = &cap.name("letter") {
      let mut stack = VecDeque::new();
      stack.push_back(letter.as_str().as_bytes()[0]);
      stacks.push(stack);
      stacks
    } else {
      stacks.push(VecDeque::new());
      stacks
    }
  })
}

fn parse_stacks(lines: &Vec<String>) -> (Vec<VecDeque<u8>>, u8) {
  let regex_stacks = Regex::new(REGEX_STACKS).unwrap();
  let regex_terminate = Regex::new(REGEX_TERMINATE).unwrap();
  let mut stacks = init_stacks(&lines[0]);
  let mut index = 2;
  for line in lines.iter().skip(1) {
    index += 1;
    if regex_terminate.is_match(line) {
      break;
    }
    regex_stacks.captures_iter(line).enumerate().for_each(|(i, cap)| {
      if let Some(letter) = &cap.name("letter") {
        stacks[i].push_front(letter.as_str().as_bytes()[0]);
      }
    });
  };
  (stacks, index)
}

#[cfg(test)]
mod tests {
  use std::collections::VecDeque;

  use super::{Instruction, parse_instructions, parse_stacks, execute_instruction, execute_instruction2, get_top_of_stacks};

  #[test]
  fn test_execute_instruction2() {
    let mut stacks = vec![
      VecDeque::from(vec![('Z' as u8), ('N' as u8), ('D' as u8)]),
      VecDeque::from(vec![('M' as u8), ('C' as u8)]),
      VecDeque::from(vec![('P' as u8)]),
    ];

    let instructions = Instruction(3, 1, 3);

    execute_instruction2(&mut stacks, &instructions);

    let expected = vec![
      VecDeque::from(vec![]),
      VecDeque::from(vec![('M' as u8), ('C' as u8)]),
      VecDeque::from(vec![('P' as u8), ('Z' as u8), ('N' as u8), ('D' as u8)]),
    ];

    assert_eq!(stacks, expected);
  }
  
  #[test]
  fn test_get_top_of_stacks() {
    let input = vec![
      VecDeque::from(vec![('C' as u8)]),
      VecDeque::from(vec![('M' as u8), ]),
      VecDeque::from(vec![('P' as u8), ('D' as u8), ('N' as u8), ('Z' as u8)]),
    ];
    
    assert_eq!(get_top_of_stacks(&input), String::from("CMZ"));
  }
  
  #[test]
  fn test_execute_instruction() {
    let mut stacks = vec![
      VecDeque::from(vec![('Z' as u8), ('N' as u8), ('D' as u8)]),
      VecDeque::from(vec![('M' as u8), ('C' as u8)]),
      VecDeque::from(vec![('P' as u8)]),
    ];

    let instructions = Instruction(3, 1, 3);

    execute_instruction(&mut stacks, &instructions);

    let expected = vec![
      VecDeque::from(vec![]),
      VecDeque::from(vec![('M' as u8), ('C' as u8)]),
      VecDeque::from(vec![('P' as u8), ('D' as u8), ('N' as u8), ('Z' as u8)]),
    ];

    assert_eq!(stacks, expected);
  }
  
  #[test]
  fn test_parse_instructions() {
    let input = &[
      String::from("move 1 from 2 to 1"),
      String::from("move 3 from 1 to 3"),
      String::from("move 2 from 2 to 1"),
      String::from("move 1 from 1 to 2"),
    ];

    let output = vec![
      Instruction(1, 2, 1),
      Instruction(3, 1, 3),
      Instruction(2, 2, 1),
      Instruction(1, 1, 2),
    ];
    assert_eq!(parse_instructions(input), output);
  }

  #[test]
  fn test_parse_stacks() {
    let input = vec![
      String::from("    [D]    "),
      String::from("[N] [C]    "),
      String::from("[Z] [M] [P]"),
      String::from(" 1   2   3 "),
    ];

    let output = vec![
      VecDeque::from(vec![('Z' as u8), ('N' as u8)]),
      VecDeque::from(vec![('M' as u8), ('C' as u8), ('D' as u8)]),
      VecDeque::from(vec![('P' as u8)]),
    ];
    assert_eq!(parse_stacks(&input), (output , 5));
  }
}