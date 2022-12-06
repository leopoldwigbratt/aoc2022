use std::error::Error;
use std::env::current_dir;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Lines};
use std::fmt::{self, Display};
use std::path::PathBuf;

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

fn relative_path(file: &str) -> Result<PathBuf, Box<dyn Error>> {
  let path = current_dir()?.join("input").join(file);
  Ok(path)
}

pub fn read_input_lines(file: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
  let path = relative_path(file)?;
  let input = File::open(path)?;
  Ok(BufReader::new(input).lines())
}

pub fn read_input_lines_ok(file: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let path = relative_path(file)?;
  let input = File::open(path)?;
  Ok(BufReader::new(input).lines().filter(|l| l.is_ok()).flatten().collect())
}

pub fn read_input_string(file: &str) -> Result<String, Box<dyn Error>> {
  let path = relative_path(file)?;
  let string = read_to_string(path)?;
  Ok(string)
}

