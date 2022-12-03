use std::{error::Error, fs::OpenOptions};

use crate::utils::{read_input_lines, SolveError};


trait GetPoints {
  fn get_points(&self) -> u32;
}

#[derive(Debug, PartialEq)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

impl GetPoints for Shape {
  fn get_points(&self) -> u32 {
      match self {
        Rock     => 1,
        Paper    => 2,
        Scissors => 3,
      }
  }
}

#[derive(Debug, PartialEq)]
enum Round {
  Victory(Shape),
  Draw(Shape),
  Defeat(Shape),
}

impl GetPoints for Round {
  fn get_points(&self) -> u32 {
      match self {
        Victory(shape) => 6 + shape.get_points(),
        Draw(shape)    => 3 + shape.get_points(),
        Defeat(shape)  => shape.get_points(),
      }
  }
}

use Shape::*;
use Round::*;

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines = read_input_lines("day2.txt")?;
  
  let sum: u32 = lines.into_iter().map(|l| parse_round(l.unwrap()).unwrap().get_points()).sum();

  let sum2: u32 = read_input_lines("day2.txt")?.map(|l| parse_round2(l.unwrap()).unwrap().get_points()).sum();

  Ok((sum.to_string(), sum2.to_string()))
}

fn parse_round(line: String) -> Result<Round, SolveError> {
  let (opponent, me) = line.split_at(1);
  let opponent = parse_shape(opponent)?;
  let me = parse_shape(me)?;

  let result = match opponent {
    Rock => match me {
      Rock     => Draw(me),
      Paper    => Victory(me),
      Scissors => Defeat(me),
    },
    Paper => match me {
      Rock     => Defeat(me),
      Paper    => Draw(me),
      Scissors => Victory(me),
    },
    Scissors => match me {
      Rock     => Victory(me),
      Paper    => Defeat(me),
      Scissors => Draw(me),
    },
  };

  Ok(result)
}

fn parse_round2(line: String) -> Result<Round, SolveError> {
  let (opponent, me) = line.split_at(1);
  let opponent = parse_shape(opponent)?;
  
  match me.trim() {
    "X" => Ok(Defeat(match opponent {
      Rock     => Scissors,
      Paper    => Rock,
      Scissors => Paper,
    })),
    "Y" => Ok(Draw(opponent)),
    "Z" => Ok(Victory(match opponent {
      Rock     => Paper,
      Paper    => Scissors,
      Scissors => Rock,
    })),
    _   => Err(SolveError::new(&format!("Failed to parse round result: {}", me))),
  }
}

fn parse_shape(s: &str) -> Result<Shape, SolveError> {
  // println!("{}", s);
  match s.trim() {
    "A" | "X" => Ok(Rock),
    "B" | "Y" => Ok(Paper),
    "C" | "Z" => Ok(Scissors),
    _         => Err(SolveError::new(&format!("Failed to parse to shape: {}", s))),
  }
}

#[cfg(test)]
mod tests {
  use crate::day2::*;
  
  #[test]
  fn test_parse_shape() {
    assert_eq!(parse_shape("A"), Ok(Rock));
    assert_eq!(parse_shape("Y"), Ok(Paper));
    assert!(parse_shape("Foo").is_err());
  }
  
  #[test]
  fn test_parse_round() {
    assert_eq!(parse_round("A Y".into()), Ok(Victory(Paper)));
  }

  #[test]
  fn test_get_points() {
    assert_eq!(Victory(Paper).get_points(), 8);
    assert_eq!(Defeat(Rock).get_points(), 1);
    assert_eq!(Draw(Scissors).get_points(), 6);
  }

  #[test]
  fn test_parse_round2() {
    assert_eq!(parse_round2("A Y".into()), Ok(Draw(Rock)));
    assert_eq!(parse_round2("B X".into()), Ok(Defeat(Rock)));
    assert_eq!(parse_round2("C Z".into()), Ok(Victory(Rock)));
  }
}