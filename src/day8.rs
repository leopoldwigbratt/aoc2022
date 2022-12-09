use std::{error::Error, collections::HashSet};

use crate::utils::read_input_string;

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let input = read_input_string("day8.txt")?;

  let (grid, side) = parse_trees(&input);
  
  let mut visible: HashSet<usize> = HashSet::with_capacity(side * side);

  (1..side - 1).for_each(|i| {
    (1..side - 1).fold((grid[side * i], grid[side * (i + 1) - 1]), |(mut left_max, mut right_max), j| {
      let left_index = side * i + j;
      let right_index = side * (i + 1) - j - 1;
      
      let left = grid[left_index];
      let right = grid[right_index];

      if left > left_max {
        left_max = left;
        visible.insert(left_index);
      }

      if right > right_max {
        right_max = right;
        visible.insert(right_index);
      }
      
      (left_max, right_max)
    });
    
    (1..side - 1).fold((grid[i], grid[side * (side - 1) + i]), |(mut top_max, mut bottom_max), j| {
      let top_index = i + side * j;
      let bottom_index = side * (side - 1 - j) + i;
      
      let top = grid[top_index];
      let bottom = grid[bottom_index];

      if top > top_max {
        top_max = top;
        visible.insert(top_index);
      }

      if bottom > bottom_max {
        bottom_max = bottom;
        visible.insert(bottom_index);
      }

      (top_max, bottom_max)
    });
  });

  let part1 = visible.len() + 4 * (side - 1);

  let part2 = (0..side).fold(0, |i_max, i| {
    std::cmp::max((0..side).fold(0, |j_max, j| {
      std::cmp::max(j_max, calculate_scenic_score(&grid, side, (i, j)))
    }), i_max)
  });

  Ok((part1.to_string(), part2.to_string()))
}

fn calculate_scenic_score(grid: &[u8], side: usize, pos: (usize, usize)) -> usize {
  let top = distance_top(grid, side, pos);
  let right = distance_right(grid, side, pos);
  let bottom = distance_bottom(grid, side, pos);
  let left = distance_left(grid, side, pos);
  
  top * right * bottom * left
}

fn distance_top(grid: &[u8], side: usize, (i, j): (usize, usize)) -> usize {
  let tree_index = i + side * j;
  let tree_height = grid[tree_index];
  
  for offset in 1..=j {
    if grid[tree_index - side * offset] >= tree_height {
      return offset;
    }
  }

  j
}

fn distance_right(grid: &[u8], side: usize, (i, j): (usize, usize)) -> usize {
  let tree_index = i + side * j;
  let tree_height = grid[tree_index];
  let max_offset = side - 1 - i;

  for offset in 1..=max_offset {
    if grid[tree_index + offset] >= tree_height {
      return offset;
    }
  }

  max_offset
}

fn distance_bottom(grid: &[u8], side: usize, (i, j): (usize, usize)) -> usize {
  let tree_index = i + side * j;
  let tree_height = grid[tree_index];
  let max_offset = side - 1 -j;

  for offset in 1..=max_offset {
    if grid[tree_index + side * offset] >= tree_height {
      return offset;
    }
  }

  max_offset
}

fn distance_left(grid: &[u8], side: usize, (i, j): (usize, usize)) -> usize {
  let tree_index = i + side * j;
  let tree_height = grid[tree_index];

  for offset in 1..=i {
    if grid[tree_index - offset] >= tree_height {
      return offset;
    }
  }

  i
}

fn initialize_grid(string: &String) -> (Vec<u8>, usize) {
  let side = string.find('\n').unwrap();
  (Vec::with_capacity(side * side), side)
}

fn parse_trees(string: &String) -> (Vec<u8>, usize) {
  let (mut grid, side) = initialize_grid(string);
  let j_length = side + 1;
  let bytes = string.as_bytes();

  (0..side).for_each(|i| {
    let start = i * j_length;
    let end = (i + 1) * j_length - 1;
    grid.extend_from_slice(&bytes[start..end]);
  });

  (grid, side)
}

#[cfg(test)]
mod tests {
  use super::{initialize_grid, parse_trees};

  #[test]
  fn test_initialize_grid() {
    let input = String::from("30373\n25512\n65332\n33549\n35390");
    let (grid, side) = initialize_grid(&input);
    assert_eq!(side, 5);
    assert_eq!(grid.capacity(), 25);
  }

  #[test]
  fn test_parse_trees() {
    let input = String::from("30373\n25512\n65332\n33549\n35390");
    // let expected = vec![b'5', b'5', b'1', b'5', b'3', b'3', b'3', b'5', b'4'];
    let expected = vec![b'3', b'0', b'3', b'7', b'3', b'2', b'5', b'5', b'1', b'2', b'6', b'5', b'3', b'3', b'2', b'3', b'3', b'5', b'4', b'9', b'3', b'5', b'3', b'9', b'0'];
    assert_eq!(parse_trees(&input), (expected, 5));
  }
}