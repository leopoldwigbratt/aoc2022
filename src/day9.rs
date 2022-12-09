use regex::Regex;

use std::{collections::{HashSet}, error::Error};

use crate::utils::read_input_lines_ok;

type Coords = (i32, i32);

#[derive(PartialEq, Clone, Copy)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(PartialEq, Clone, Copy)]
struct Motion(Direction, isize);

struct Rope {
  knots: [Coords; 10],
  visited: HashSet<Coords>,
}

impl Rope {
  pub fn new() -> Self {
    Self {
      knots: [(0, 0); 10],
      visited: HashSet::with_capacity(3000),
    }
  }

  pub fn visited(&self) -> usize {
    self.visited.len()
  }

  pub fn apply_motion(&mut self, motion: Motion) {
    let Motion(direction, steps) = motion;

    (1..=steps).for_each(|_| {
      self.step(direction);
    });
  }

  fn step(&mut self, direction: Direction) {
    let head = self.knots[0];
    
    let head = match direction {
      Direction::Up    => (head.0, head.1 + 1),
      Direction::Right => (head.0 + 1, head.1),
      Direction::Down  => (head.0, head.1 - 1),
      Direction::Left  => (head.0 - 1, head.1),
    };

    self.knots[0] = head;

    (1..10).fold(head, |head, i| {
      let (tail, _) = match direction {
        Direction::Up    => step_tail(self.knots[i], head),
        Direction::Right => step_tail(self.knots[i], head),
        Direction::Down  => step_tail(self.knots[i], head),
        Direction::Left  => step_tail(self.knots[i], head),
      };

      self.knots[i] = tail;
      
      if i == 9 {
        self.visited.insert(tail);
      }

      tail
    });
  }
}

const REGEX: &str = r"^(?P<direction>(U|R|D|L)) (?P<steps>\d+)$";

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
    let lines = read_input_lines_ok("day9.txt")?;

    let regex = Regex::new(REGEX).unwrap();

    let mut visited: HashSet<Coords> = HashSet::with_capacity(6000);

    let motions = parse_directions(&lines, &regex);

    motions.iter().fold(((0, 0), (0, 0)), |(tail, head), motion| {
      apply_motion(tail, head, motion, &mut visited)
    });

    let part1 = visited.len();

    let mut rope = Rope::new();

    motions.iter().for_each(|motion| {
      rope.apply_motion(*motion);
    });

    let part2 = rope.visited();

    Ok((part1.to_string(), part2.to_string()))
}



fn apply_motion(tail: Coords, head: Coords, motion: &Motion, visited: &mut HashSet<Coords>) -> (Coords, Coords) {
  let Motion(direction, steps) = motion;

  (1..=steps.abs() as usize).fold((tail, head), |(tail, head), _| {
    let (tail, head) = step(tail, head, *direction);
    visited.insert(tail);
    (tail, head)
  })
}

fn step(tail: Coords, head: Coords, direction: Direction) -> (Coords, Coords) {
   match direction {
    Direction::Up    => step_tail(tail, (head.0, head.1 + 1)),
    Direction::Right => step_tail(tail, (head.0 + 1, head.1)),
    Direction::Down  => step_tail(tail, (head.0, head.1 - 1)),
    Direction::Left  => step_tail(tail, (head.0 - 1, head.1)),
  }
}

fn step_tail(tail: Coords, head: Coords) -> (Coords, Coords) {
  let delta_x = head.0 - tail.0;
  let delta_y = head.1 - tail.1;
  
  if delta_x.abs() > 1 {
    if delta_y.abs() > 0 {
      return ((tail.0 + 1 * delta_x.signum(), tail.1 + 1 * delta_y.signum()), head);
    }

    return ((tail.0 + 1 * delta_x.signum(), tail.1), head);
  }

  if delta_y.abs() > 1 {
    if delta_x.abs() > 0 {
      return ((tail.0 + 1 * delta_x.signum(), tail.1 + 1 * delta_y.signum()), head);
    }

    return ((tail.0, tail.1 + 1 * delta_y.signum()), head);
  }

  (tail, head) 
}

fn parse_directions(lines: &Vec<String>, regex: &Regex) -> Vec<Motion> {
  lines.iter().map(|line| parse_direction(line, regex)).collect()
}

fn parse_direction(line: &String, regex: &Regex) -> Motion {
  use Direction::*;

  let captures = regex.captures(line).unwrap();

  let steps = captures.name("steps").unwrap().as_str().parse::<isize>().unwrap();

  match captures.name("direction").unwrap().as_str() {
    "U" => Motion(Up, steps),
    "R" => Motion(Right, steps),
    "D" => Motion(Down, steps),
    "L" => Motion(Left, steps),
    _   => unreachable!(),
  }
}

#[cfg(test)]
mod tests {
  use super::{step_tail, step, Direction::*};

  #[test]
  fn test_step() {
    // Move head and tail follows (straight line)
    assert_eq!(step((2, 3), (2, 4), Up), ((2, 4), (2, 5)));
    assert_eq!(step((3, 1), (4, 1), Right), ((4, 1), (5, 1)));
    assert_eq!(step((-1, 5), (-1, 4), Down), ((-1, 4), (-1, 3)));
    assert_eq!(step((-2, -3), (-3, -3), Left), ((-3, -3), (-4, -3)));

    // Move head and tail follow (diagonally)
    assert_eq!(step((3, 1), (4, 2), Up), ((4, 2), (4, 3)));
 

    // Move head and tail is stationary (same start coords)
    assert_eq!(step((4, 5), (4, 5), Up), ((4, 5), (4, 6)));
    assert_eq!(step((3, 4), (3, 4), Right), ((3, 4), (4, 4)));
    assert_eq!(step((-2, 6), (-2, 6), Down), ((-2, 6), (-2, 5)));
    assert_eq!(step((-1, 0), (-1, 0), Left), ((-1, 0), (-2, 0)));
  }
  
  #[test]
  fn test_step_tail() {
    assert_eq!(step_tail((0, 0), (0, 2)), ((0, 1), (0, 2)));
    assert_eq!(step_tail((0, -1), (0, 1)), ((0, 0), (0, 1)));
    assert_eq!(step_tail((-2, -3), (-2, -5)), ((-2, -4), (-2, -5)));
    
    assert_eq!(step_tail((2, 3), (1, 3)), ((2, 3), (1, 3)));
    assert_eq!(step_tail((5, 6), (5, 7)), ((5, 6), (5, 7)));
    assert_eq!(step_tail((-4, -2), (-4, -1)), ((-4, -2), (-4, -1)));

    assert_eq!(step_tail((3, 2), (5, 2)), ((4, 2), (5, 2)));
    assert_eq!(step_tail((-3, -5), (-5, -5)), ((-4, -5), (-5, -5)));
    assert_eq!(step_tail((-1, 3), (1, 3)), ((0, 3), (1, 3)));
  }
}