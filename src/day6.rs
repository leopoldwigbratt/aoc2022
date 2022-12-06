use std::error::Error;
use std::collections::HashMap;
use std::cmp::max;

use crate::utils::read_input_string;

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let buffer = read_input_string("day6.txt")?;
  
  let mut map1: HashMap<char, usize> = HashMap::with_capacity(4);
  let mut map2: HashMap<char, usize> = HashMap::with_capacity(14);
  
  let part1 = check_buffer(&buffer, 4, &mut map1);
  let part2 = check_buffer(&buffer, 14, &mut map2);

  Ok((part1.to_string(), part2.to_string()))
}

fn check_buffer(buffer: &String, marker_length: usize, map: &mut HashMap<char, usize>) -> usize {
  let mut start_index = 0;
  
  if start_index + marker_length > buffer.len() { panic!("The size of the buffer ({}) is less than the start-of-packet-marker ({})", buffer.len(), marker_length); }
  
  let mut slice = &buffer[start_index..(start_index + marker_length)];

  while let (false, offset) = check_slice(slice, map) {
    start_index += offset;
    let end_index = start_index + marker_length;

    if end_index > buffer.len() { panic!("The size of the buffer ({}) is less than the start-of-packet-marker ({})", buffer.len(), marker_length); }

    slice = &buffer[start_index..end_index];
  }
  
  start_index + marker_length
}

fn check_slice(slice: &str, map: &mut HashMap<char, usize>) -> (bool, usize) {
  let result = slice.chars().enumerate().fold((true, 1), |(check, offset), (current, char)| {
    if let Some(last) = map.get(&char) {
      (false, max(last + 1, offset))
    } else {
      map.insert(char, current);

      (check, offset)
    }
  });

  map.clear();

  result
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::{check_slice, check_buffer};

  #[test]
  fn test_check_buffer() {
    let mut map1: HashMap<char, usize> = HashMap::with_capacity(4);
    let mut map2: HashMap<char, usize> = HashMap::with_capacity(14);

    let buffer1 = &String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    let buffer2 = &String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
    let buffer3 = &String::from("nppdvjthqldpwncqszvftbrmjlhg");
    let buffer4 = &String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    let buffer5 = &String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

    assert_eq!(check_buffer(&buffer1, 4, &mut map1), 7);
    assert_eq!(check_buffer(&buffer2, 4, &mut map1), 5);
    assert_eq!(check_buffer(&buffer3, 4, &mut map1), 6);
    assert_eq!(check_buffer(&buffer4, 4, &mut map1), 10);
    assert_eq!(check_buffer(&buffer5, 4, &mut map1), 11);
    
    assert_eq!(check_buffer(&buffer1, 14, &mut map2), 19);
    assert_eq!(check_buffer(&buffer2, 14, &mut map2), 23);
    assert_eq!(check_buffer(&buffer3, 14, &mut map2), 23);
    assert_eq!(check_buffer(&buffer4, 14, &mut map2), 29);
    assert_eq!(check_buffer(&buffer5, 14, &mut map2), 26);
  }
    
  #[test]
  fn test_check_slice() {
    let mut map: HashMap<char, usize> = HashMap::with_capacity(4);
    
    let string = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    let string2 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    
    assert_eq!(check_slice(&string[2..6], &mut map), (false, 1));
    assert_eq!(check_slice(&string[3..7], &mut map), (true, 1)); // test successful check
    assert_eq!(check_slice(&string2[4..8], &mut map), (false, 3)); // test unsuccessful check with jump "fwzz"([4..8]) -> "zqfr"([7..11])
  }
}