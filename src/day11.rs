use lazy_static::lazy_static;
use regex::Regex;

use std::{error::Error, collections::VecDeque};

use crate::utils::read_input_string;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
  Mul(u64),
  Add(u64),
  Square,
}

#[derive(Clone, Debug, PartialEq)]
struct Monkey {
  items: VecDeque<u64>,
  operation: Operation,
  test: u64,
  if_true: usize,
  if_false: usize,
  activity: usize,
}

impl Monkey {
  fn inspect<const PART2: bool>(&mut self, lcm: u64) -> Option<(usize, u64)> {
    let Some(item) = self.items.pop_front() else {
      return None;
    };

    let mut worry = match self.operation {
      Operation::Add(n) => item + n,
      Operation::Mul(n) => item * n,
      Operation::Square => item * item,
    };


    if !PART2 { worry /=  3; }
    worry %= lcm;

    self.activity += 1;
    
    if worry % self.test == 0 {
      Some((self.if_true, worry))
    } else {
      Some((self.if_false, worry))
    }
  }
}

const MONKEY_PATTERN: &str = r"(?P<monkey>Monkey (?P<monkey_id>\d+):\n\s+Starting items: (?P<items>((\d+)(, )?)+)\n\s+Operation: new = old (?P<operation>\*|\+) (?P<rhs>old|\d+)\n\s+Test: divisible by (?P<divisible>\d+)\n\s+If true: throw to monkey (?P<true>\d+)\n\s+If false: throw to monkey (?P<false>\d+))";
const ITEM_PATTERN: &str = r"((?P<item>\d+)(, )?)";

lazy_static! {
  static ref MONKEY_REGEX: Regex = Regex::new(MONKEY_PATTERN).unwrap();
  static ref ITEM_REGEX: Regex = Regex::new(ITEM_PATTERN).unwrap();
}

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let input = read_input_string("day11.txt")?;

  let mut monkeys = parse_monkeys(&input);
  
  let lcm = monkeys.iter().map(|m| m.test).product::<u64>();

  let mut monkeys2 = monkeys.clone();

  (0..20).for_each(|_| step_round::<false>(&mut monkeys, lcm));

  monkeys.sort_by(|a, b| b.activity.partial_cmp(&a.activity).unwrap());

  let part1 = monkeys.iter().take(2).map(|monkey| monkey.activity).product::<usize>();

  (0..10000).for_each(|_| step_round::<true>(&mut monkeys2, lcm));

  monkeys2.sort_by(|a, b| b.activity.partial_cmp(&a.activity).unwrap());

  let part2 = monkeys2.iter().take(2).map(|m| m.activity).product::<usize>();

  Ok((part1.to_string(), part2.to_string()))
}

fn step_round<const PART2: bool>(monkeys: &mut Vec<Monkey>, lcm: u64) {
  for from in 0..monkeys.len() {
    while let Some((to, item)) = monkeys[from].inspect::<PART2>(lcm) {
      monkeys[to].items.push_back(item);
    }
  }
}

fn parse_monkeys(input: &String) -> Vec<Monkey> {
  MONKEY_REGEX.captures_iter(input).map(|caps| { 
    let items = ITEM_REGEX
      .captures_iter(caps.name("items").unwrap().as_str())
      .map(|m| {
        m.get(2).unwrap().as_str().parse().unwrap()
      })
      .collect();

    let rhs = caps.name("rhs").unwrap().as_str();

    Monkey {
      items,
      operation: match caps.name("operation").unwrap().as_str() {
        "*" => match rhs {
          "old" => Operation::Square,
          n => Operation::Mul(n.parse().unwrap()),
        },
        "+" => Operation::Add(rhs.parse().unwrap()),
        _ => unreachable!(),
      },
      activity: 0,
      test: caps.name("divisible").unwrap().as_str().parse().unwrap(),
      if_true: caps.name("true").unwrap().as_str().parse().unwrap(),
      if_false: caps.name("false").unwrap().as_str().parse().unwrap(),
    }
  }).collect()
}

#[cfg(test)]
mod tests {
  use std::collections::VecDeque;

  use super::{parse_monkeys, Operation, Monkey, step_round};

  #[test]
  fn test_part2() {
    let mut input = vec![
      Monkey {
        items: VecDeque::from(vec![79, 98]),
        operation: Operation::Mul(19),
        test: 23,
        if_true: 2,
        if_false: 3,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![54, 65, 75, 74]),
        operation: Operation::Add(6),
        test: 19,
        if_true: 2,
        if_false: 0,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![79, 60, 97]),
        operation: Operation::Square,
        test: 13,
        if_true: 1,
        if_false: 3,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![74]),
        operation: Operation::Add(3),
        test: 17,
        if_true: 0,
        if_false: 1,
        activity: 0,
      },
    ];

    let lcm = input.iter().map(|m| m.test).product::<u64>();

    (0..20).for_each(|_| step_round::<true>(&mut input, lcm));
    assert_eq!(input.iter().map(|m| m.activity).collect::<Vec<_>>(), vec![99, 97, 8, 103]);
    
    (20..1000).for_each(|_| step_round::<true>(&mut input, lcm));
    assert_eq!(input.iter().map(|m| m.activity).collect::<Vec<_>>(), vec![5204, 4792, 199, 5192]);

    (1000..2000).for_each(|_| step_round::<true>(&mut input, lcm));
    assert_eq!(input.iter().map(|m| m.activity).collect::<Vec<_>>(), vec![10419, 9577, 392, 10391]);

    (2000..10000).for_each(|_| step_round::<true>(&mut input, lcm));
    assert_eq!(input.iter().map(|m| m.activity).collect::<Vec<_>>(), vec![52166, 47830, 1938, 52013]);

    input.sort_by(|a, b| b.activity.partial_cmp(&a.activity).unwrap());

    assert_eq!(input.iter().take(2).map(|m| m.activity).product::<usize>(), 2_713_310_158);
  }
  
  #[test]
  fn test_step_round() {
    let mut input = vec![
      Monkey {
        items: VecDeque::from(vec![79, 98]),
        operation: Operation::Mul(19),
        test: 23,
        if_true: 2,
        if_false: 3,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![54, 65, 75, 74]),
        operation: Operation::Add(6),
        test: 19,
        if_true: 2,
        if_false: 0,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![79, 60, 97]),
        operation: Operation::Square,
        test: 13,
        if_true: 1,
        if_false: 3,
        activity: 0,
      },
      Monkey {
        items: VecDeque::from(vec![74]),
        operation: Operation::Add(3),
        test: 17,
        if_true: 0,
        if_false: 1,
        activity: 0,
      },
    ];

    let lcm = input.iter().map(|m| m.test).product::<u64>();
    
    let expected = vec![
      vec![
        vec![20, 23, 27, 26],
        vec![2080, 25, 167, 207, 401, 1046],
        vec![],
        vec![],
      ],
      vec![
        vec![695, 10, 71, 135, 350],
        vec![43, 49, 58, 55, 362],
        vec![],
        vec![],
      ],
      vec![
        vec![16, 18, 21, 20, 122],
        vec![1468, 22, 150, 286, 739],
        vec![],
        vec![],
      ],
      vec![
        vec![491, 9, 52, 97, 248, 34],
        vec![39, 45, 43, 258],
        vec![],
        vec![],
      ],
      vec![
        vec![15, 17, 16, 88, 1037],
        vec![20, 110, 205, 524, 72],
        vec![],
        vec![],
      ],
      vec![
        vec![8, 70, 176, 26, 34],
        vec![481, 32, 36, 186, 2190],
        vec![],
        vec![],
      ],
      vec![
        vec![162, 12, 14, 64, 732, 17],
        vec![148, 372, 55, 72],
        vec![],
        vec![],
      ],
      vec![
        vec![51, 126, 20, 26, 136],
        vec![343, 26, 30, 1546, 36],
        vec![],
        vec![],
      ],
      vec![
        vec![116, 10, 12, 517, 14],
        vec![108, 267, 43, 55, 288],
        vec![],
        vec![],
      ],
      vec![
        vec![91, 16, 20, 98],
        vec![481, 245, 22, 26, 1092, 30],
        vec![],
        vec![],
      ],
      vec![
        vec![83, 44, 8, 184, 9, 20, 26, 102],
        vec![110, 36],
        vec![],
        vec![],
      ],
      vec![
        vec![10, 12, 14, 26, 34],
        vec![245, 93, 53, 199, 115],
        vec![],
        vec![],
      ],
    ];

    (0..10).for_each(|i| {
      step_round::<false>(&mut input, lcm);
      let output = input.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
      assert_eq!(output, expected[i]);
    });

    (10..15).for_each(|_| step_round::<false>(&mut input, lcm));
    let output = input.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
    assert_eq!(output, expected[10]);

    (15..20).for_each(|_| step_round::<false>(&mut input, lcm));
    let output = input.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
    assert_eq!(output, expected[11]);
  }
  
  #[test]
  fn test_parse_monkeys() {
    let input = String::from("Monkey 1:\n    Starting items: 95, 88, 75, 81, 91, 67, 65, 84\n    Operation: new = old * 11\n    Test: divisible by 7\n      If true: throw to monkey 3\n      If false: throw to monkey 4");
    let output = parse_monkeys(&input);
    let expected = Monkey {
      items: VecDeque::from(vec![95, 88, 75, 81, 91, 67, 65, 84]),
      operation: Operation::Mul(11),
      test: 7,
      if_true: 3,
      if_false: 4,
      activity: 0,
    };
    assert_eq!(output[0], expected);
  }
}