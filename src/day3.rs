use std::error::Error;
use std::collections::{HashSet};

use crate::utils::read_input_lines;

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines = read_input_lines("day3.txt")?.filter(|l| l.is_ok()).flatten().collect::<Vec<_>>();
  
  let part1: u32 = lines.iter().map(|l| get_points(get_dup_item(l))).sum();

  let group_of_three = group(&lines);

  let part2: u32 = unique(&group_of_three);

  Ok((part1.to_string(), part2.to_string()))
}

fn unique(group: &Vec<Vec<&String>>) -> u32 {
  group.iter().map(|v| {
    let mut set = HashSet::new();
    let mut set2 = HashSet::new();
    v[0].chars().for_each(|c| { set.insert(c); });
    v[1].chars().filter(|c| set.contains(&c)).for_each(|c| { set2.insert(c); });
    get_points(v[2].chars().filter(|c| set2.contains(&c)).next().unwrap())
  }).sum()
}

fn group(lines: &Vec<String>) -> Vec<Vec<&String>> {
  lines.iter().enumerate().fold(Vec::new(), |mut v, (i, l)| {
    if i % 3 == 0 {
      v.push(Vec::new());
    }
    let last = v.last_mut().unwrap();
    last.push(l);
    v
  })
}

fn get_points(c: char) -> u32 {
  if c.is_uppercase() {
    c as u32 - 38
  } else {
    c as u32 - 96
  }
}

fn get_dup_item(rucksack: &String) -> char {
  let rucksack = rucksack.trim();

  let first_half = rucksack.chars().take(rucksack.len() / 2);
  let second_half = rucksack.chars().skip(rucksack.len() / 2);

  let mut uniques: HashSet<char> = HashSet::new();

  first_half.for_each(|i| {uniques.insert(i);});
  let x = second_half.reduce(|i1, i2| if uniques.contains(&i2) { i2 } else { i1 } );
  
  x.unwrap()
}

#[cfg(test)]
mod tests {
  use super::{get_dup_item, get_points, group, unique};

  #[test]
  fn test_unique() {
    assert_eq!(unique(&vec![
      vec![
        &String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        &String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        &String::from("PmmdzqPrVvPwwTWBwg")
      ], 
      vec![
        &String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        &String::from("ttgJtRGJQctTZtZT"),
        &String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
      ]
    ]), 70);
  }
    
  #[test]
  fn test_get_dup_item() {
    assert_eq!(get_dup_item(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_string()), 'p');
    assert_eq!(get_dup_item(&"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()), 'L');
    assert_eq!(get_dup_item(&"PmmdzqPrVvPwwTWBwg".to_string()), 'P');
  }

  #[test]
  fn test_get_points() {
    assert_eq!(get_points('A'), 27);
    assert_eq!(get_points('a'), 1);
    assert_eq!(get_points('c'), 3);
  }

  #[test]
  fn test_group() {
    let input = vec![
      String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
      String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
      String::from("PmmdzqPrVvPwwTWBwg"),
      String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
      String::from("ttgJtRGJQctTZtZT"),
      String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
    ];

    assert_eq!(group(&input), vec![
      vec![
        &String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        &String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        &String::from("PmmdzqPrVvPwwTWBwg")
      ], 
      vec![
        &String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        &String::from("ttgJtRGJQctTZtZT"),
        &String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
      ]
    ]);
  }
}