use regex::Regex;

use std::{collections::HashMap, error::Error};

use crate::utils::read_input_lines_ok;

#[derive(PartialEq, Debug)]
enum LineResult {
    Cd(String),
    File(usize),
}

const REGEX_PARSE_LINE: &str = r"^(?P<cd>\$ cd ((\w|\.|/)+))|(?P<file>([0-9]+) (\w|\.)+)$";

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
    let lines = read_input_lines_ok("day7.txt")?;

    let regex = Regex::new(REGEX_PARSE_LINE).unwrap();

    let results: Vec<LineResult> = lines
        .iter()
        .filter_map(|line| parse_line(line, &regex))
        .collect();

    let dirs = get_dirs(results);

    let part1: usize = dirs.values().filter(|size| **size <= 100000).sum();

    let space_required = 30000000 - (70000000 - dirs.get(&String::from("/")).unwrap());

    let part2 = dirs
        .values()
        .filter(|v| **v > space_required)
        .fold(usize::MAX, |min, val| std::cmp::min(*val, min));

    Ok((part1.to_string(), part2.to_string()))
}

fn get_dirs(results: Vec<LineResult>) -> HashMap<String, usize> {
    let (_, map): (Vec<String>, HashMap<String, usize>) = results.iter().fold(
        (Vec::new(), HashMap::new()),
        |(mut current_path, mut map), lr| {
            match lr {
                LineResult::Cd(path) => match path.as_str() {
                    ".." => {
                        current_path.pop();
                    }
                    p => current_path.push(String::from(p)),
                },
                LineResult::File(size) => add_to_dirs(&mut current_path, &mut map, *size),
            }
            (current_path, map)
        },
    );
    map
}

fn add_to_dirs(current_path: &mut Vec<String>, map: &mut HashMap<String, usize>, size: usize) {
    (0..current_path.len()).for_each(|index| {
        let path = String::from(&current_path[..=index].join("/"));
        *map.entry(path).or_insert(0) += size;
    });
}

fn parse_line(line: &String, regex: &Regex) -> Option<LineResult> {
    let Some(captures) = regex.captures(line) else {
    return None;
  };

    if captures.name("cd").is_some() {
        return Some(LineResult::Cd(
            captures.get(2).unwrap().as_str().to_string(),
        ));
    }

    if captures.name("file").is_some() {
        return Some(LineResult::File(
            captures.get(5).unwrap().as_str().parse().unwrap(),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use std::collections::HashMap;

    use super::{add_to_dirs, get_dirs, parse_line, LineResult, REGEX_PARSE_LINE};

    #[test]
    fn test_add_to_dirs() {
        let mut current_path = vec![
            String::from("/"),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];

        let mut map = HashMap::new();
        add_to_dirs(&mut current_path, &mut map, 1);
        map.iter().for_each(|(k, v)| match k.as_str() {
            "/" => assert_eq!(*v, 1),
            "//a" => assert_eq!(*v, 1),
            "//a/b" => assert_eq!(*v, 1),
            "//a/b/c" => assert_eq!(*v, 1),
            "//a/b/c/d" => assert_eq!(*v, 1),
            "//a/b/c/d/e" => assert_eq!(*v, 1),
            _ => unreachable!("Should not reach"),
        });
    }

    #[test]
    fn test_sum_dirs() {
        //                         /=13
        //          /a=6             2            /b=5
        //   /aa=3   2   /ab=1                     5
        //     3           1

        let input = vec![
            LineResult::Cd(String::from("/")),
            LineResult::File(2),
            LineResult::Cd(String::from("a")),
            LineResult::File(2),
            LineResult::Cd(String::from("aa")),
            LineResult::File(3),
            LineResult::Cd(String::from("..")),
            LineResult::Cd(String::from("ab")),
            LineResult::File(1),
            LineResult::Cd(String::from("..")),
            LineResult::Cd(String::from("..")),
            LineResult::Cd(String::from("b")),
            LineResult::File(5),
        ];

        assert_eq!(get_dirs(input).values().sum::<usize>(), 28);
    }

    #[test]
    fn test_parse_command() {
        let regex = Regex::new(REGEX_PARSE_LINE).unwrap();
        assert_eq!(
            parse_line(&String::from("$ cd .."), &regex),
            Some(LineResult::Cd("..".to_string()))
        );
        assert_eq!(
            parse_line(&String::from("123 asd.asd"), &regex),
            Some(LineResult::File(123))
        );
    }
}
