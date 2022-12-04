use std::error::Error;

use crate::utils::read_input_lines_ok;

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines = read_input_lines_ok("day4.txt")?;
  let parsed = lines.iter().map(|l| parse_pairs(l));
  let count1 = parsed.clone().filter(|(left, right)| contains(left, right)).count();
  let count2 = parsed.filter(|(left, right)| overlap(left, right)).count();
  Ok((count1.to_string(), count2.to_string()))
}

fn overlap(left: &(u32, u32), right: &(u32, u32)) -> bool {
  let (ls, le) = left;
  let (rs, re) = right;
  if ls == rs {
    return true;
  }
  if ls < rs {
    return rs <= le;
  }
  return ls <= re;
}

fn contains(left: &(u32, u32), right: &(u32, u32)) -> bool {
  let (ls, le) = left;
  let (rs, re) = right;
  if ls == rs {
    return true;
  }
  if ls < rs {
    return le >= re;
  }
  return re >= le;
}

fn parse_range(s: &str) -> (u32, u32) {
  let (start, end) = s.split_once('-').unwrap();
  (start.parse().unwrap(), end.parse().unwrap())
}

fn parse_pairs(line: &str) -> ((u32, u32), (u32, u32)) {
  let (left, right) = line.split_once(',').unwrap();
  (parse_range(left), parse_range(right))
}

#[cfg(test)]
mod tests {
  use super::{parse_range, parse_pairs, contains, overlap};

  #[test]
  fn test_overlap() {
    assert!(overlap(&(2, 8), &(3, 7)));
    assert!(overlap(&(2, 8), &(8, 9)));
    assert!(overlap(&(2, 8), &(2, 7)));
    assert!(overlap(&(3, 8), &(8, 9)));
    assert!(overlap(&(2, 8), &(2, 7)));
    assert!(overlap(&(3, 5), &(2, 4)));
  }
  
  #[test]
  fn test_contains() {
    assert!(contains(&(2, 8), &(3, 7)));
    assert!(contains(&(5, 7), &(1, 8)));
    assert!(contains(&(6, 6), &(4, 6)));
    assert!(contains(&(4, 7), &(4, 8)));
    assert!(!contains(&(2, 4), &(6, 8)));
  }

  #[test]
  fn test_parse_range() {
    assert_eq!(parse_range("2-4"), (2, 4));
    assert_eq!(parse_range("123-432"), (123, 432));
  }

  #[test]
  fn test_parse_pairs() {
    assert_eq!(parse_pairs("2-4,6-8"), ((2, 4), (6, 8)));
  }
}