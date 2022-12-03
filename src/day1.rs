use std::error::Error;

use crate::utils::{read_input_lines, SolveError};

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let lines: Vec<Result<String, std::io::Error>> = read_input_lines("day1.txt")?.collect();

  let elves = accumulate_elves(lines);

  let max = elves.iter().max();

  let mut top_three = elves.clone();
  top_three.sort_by(|a, b| b.partial_cmp(a).unwrap());
  let top_three: u32 = top_three.iter().take(3).sum();

  if let Some(max) = max {
    return Ok(((*max).to_string(), top_three.to_string()));
  }

  Err(Box::new(SolveError::new("Failed to calculate max calories"))) 
}

fn accumulate_elves(lines: Vec<Result<String, std::io::Error>>) -> Vec<u32> {
  lines.into_iter().fold(Vec::new(), |mut v,r| {
    let s = r.unwrap();
    if s.len() == 0 { 
      v.push(0);
      return v
    };
    if let Ok(n) = s.parse::<u32>() {
      if v.len() == 0 {
        v.push(n);
        return v;
      }
      let i = v.len() - 1;
      v[i] += n;
      return v
    }
    v
  })
}

#[cfg(test)]
mod tests {
    use super::accumulate_elves;

  #[test]
  fn test_accumulate_elves() {
    let lines = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
      .split('\n')
      .map(|x| Ok(x.to_string()))
      .collect();
      
    assert_eq!(accumulate_elves(lines), vec![6000, 4000, 11000, 24000, 10000]);

    let lines = "\n"
      .split('\n')
      .map(|x| Ok(x.to_string()))
      .collect();
    assert_eq!(accumulate_elves(lines), vec![0, 0]);
  }
}


