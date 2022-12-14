use std::{error::Error, fmt::{Debug, Formatter, self}, collections::VecDeque};

use crate::utils::read_input_lines_ok;

#[derive(Clone, PartialEq)]
struct List(pub VecDeque<ListItem>);

impl Debug for List {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self.0.iter()).finish()
  }
}

#[derive(Clone, PartialEq)]
enum ListItem {
  Item(u32),
  List(List),
}

impl ListItem {
  pub fn to_u32(&self) -> u32 {
    match self {
      Self::Item(n) => *n,
      Self::List(_) => panic!("Cannot unpack List"),
    }
  }
}

impl Debug for ListItem {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Self::Item(int) => int.to_string(),
      Self::List(list) => format!("{list:?}"),
    })
  }
}

impl List {
  pub fn new() -> Self {
    List(VecDeque::from(vec![]))
  }

  pub fn from_vec(vec: Vec<ListItem>) -> Self {
    List(VecDeque::from(vec))
  }

  pub fn push(&mut self, item: ListItem) {
    self.0.push_back(item);
  }
}

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let input = read_input_lines_ok("day13.txt")?;

  let pairs = parse_pairs(&input);

  let part1 = pairs
    .into_iter()
    .enumerate()
    .fold(0, |mut count, (index, pair)| {
      if compare_packets(pair) {
        println!("Right: {}", index + 1);
        count += index + 1;
      }

      count
  });

  let part2 = "";
  
  Ok((part1.to_string(), part2.to_string()))
}

fn compare_lists(left: &mut List, right: &mut List) -> bool {
  while let Some(mut left) = left.0.pop_front() {
    let Some(mut right) = right.0.pop_front() else {
      return false;
    };

    if !compare_list_items(&mut left, &mut right) {
      return false;
    }
  }
  
  true
}

fn compare_list_items(left: &mut ListItem, right: &mut ListItem) -> bool {
  match left {
    item @ ListItem::Item(_) => match right {
      ListItem::Item(right) => {
        let ret = item.to_u32() <= *right;
        // println!("1: {ret}");
        ret
      },
      ListItem::List(right) => {
        let ret = compare_lists(&mut List::from_vec(vec![item.clone()]), right);
        // println!("2: {ret}");
        ret
      },
    },
    ListItem::List(left) => match right {
      item @ ListItem::Item(_) => {
        dbg!(&left);
        dbg!(&item);
        let ret = compare_lists(left, &mut List::from_vec(vec![item.clone()]));
        // println!("3: {ret}");
        ret 
      },
      ListItem::List(right) => {
        let ret = compare_lists(left, right);
        // println!("4: {ret}");
        ret 
      },
    }
  }
}

fn compare_packets((mut left, mut right): (List, List)) -> bool {
  compare_lists(&mut left, &mut right)
}

fn parse_packet(line: &String) -> List {
  line.chars().skip(1).fold((List::new(), Vec::<List>::new()), |(mut list, mut stack), char| {
    if char == ',' {
      return (list, stack);
    } else if char.is_numeric() {
      if let Some(last) = stack.last_mut() {
        last.push(ListItem::Item(char as u32 - b'0' as u32));
      } else {
        list.push(ListItem::Item(char as u32 - b'0' as u32));
      }
      return (list, stack);
    } else if char == '[' {
      stack.push(List::new());
      return (list, stack)
    } else if char == ']' {
      let Some(x) = stack.pop() else {
        return (list, stack);
      };
      if let Some(last) = stack.last_mut() {
        last.push(ListItem::List(x));
      } else {
        list.push(ListItem::List(x));
      }

    }
    (list, stack)
  }).0
}

fn parse_pairs(lines: &Vec<String>) -> Vec<(List, List)> {
  let mut pairs = Vec::with_capacity((lines.len() + 1) * 2 / 3);
  for i in (0..lines.len() + 1).step_by(3) {
    pairs.push((parse_packet(&lines[i]), parse_packet(&lines[i + 1])));
  }

  pairs
}

#[cfg(test)]
mod tests {
  use super::{List, ListItem, parse_packet, compare_packets, parse_pairs};

  fn item(n: u32) -> ListItem {
    ListItem::Item(n + b'0' as u32)
  }

  fn list(v: Vec<ListItem>) -> ListItem {
    ListItem::List(List::from_vec(v))
  }

  #[test]
  fn test_compare_packets() {
    // assert!(compare_packets((parse_packet(&String::from("[[1],[2,3,4]]")), parse_packet(&String::from("[[1],4]")))))
  }

  #[test]
  fn test_parse_pairs() {
    let input = vec![
      String::from("[1,1,3,1,1]"),
      String::from("[1,1,5,1,1]"),
      String::from(""),
      String::from("[[1],[2,3,4]]"),
      String::from("[[1],4]"),
      // String::from("\n"),
      // String::from("[9]\n"),
      // String::from("[[8,7,6]]\n"),
      // String::from("\n"),
      // String::from("[[4,4],4,4]\n"),
      // String::from("[[4,4],4,4,4]\n"),
      // String::from("\n"),
      // String::from("[7,7,7,7]\n"),
      // String::from("[7,7,7]\n"),
      // String::from("\n"),
      // String::from("[]\n"),
      // String::from("[3]\n"),
      // String::from("\n"),
      // String::from("[[[]]]\n"),
      // String::from("[[]]\n"),
      // String::from("\n"),
      // String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]\n"),
      // String::from("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
    ];
    
    let expected = vec![
      (List::from_vec(vec![item(1), item(1), item(3), item(1), item(1)]), List::from_vec(vec![item(1), item(1), item(5), item(1), item(1)])),
      (List::from_vec(vec![list(vec![item(1)]), list(vec![item(2), item(3), item(4)])]), List::from_vec(vec![list(vec![item(1)]), item(4)])),
    ];

    assert_eq!(parse_pairs(&input), expected);
  }
  
  #[test]
  fn test_parse_packet() {
    use ListItem::Item;

    let input = vec![
      String::from("[1,1,3,1,1]"),
      String::from("[[1],[2,3,4]]"),
      String::from("[9]"),
      String::from("[[8,7,6]]"),
    ];

    let expected = vec![
      List::from_vec(vec![item(1), item(1), item(3), item(1), item(1)]),
      List::from_vec(vec![list(vec![item(1)]), list(vec![item(2), item(3), item(4)])]),
      List::from_vec(vec![item(9)]),
      List::from_vec(vec![list(vec![item(8), item(7), item(6)])]),
    ];

    expected.iter().zip(input.iter()).for_each(|(expected, input)| {
      assert_eq!(parse_packet(input), *expected);
    });
  }
}