use std::error::Error;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub struct SolveError {
  msg: String
}

impl SolveError {
    pub fn new(msg: &str) -> Self { 
      Self { msg: msg.to_string() }
    }
}

impl Display for SolveError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SolveError: {}", self.msg)
  }
}

impl Error for SolveError {}

pub fn read_input_lines(file: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
  let path = current_dir()?.join("input").join(file);
  let input = File::open(path)?;
  Ok(BufReader::new(input).lines())
}



