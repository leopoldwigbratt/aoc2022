use std::{error::Error, collections::VecDeque};

use crate::utils::read_input_string;

#[derive(Debug, PartialEq, Clone)]
struct Graph {
  width: usize,
  height: usize,
  vertices: Vec<u8>,
  edges: Vec<(bool, bool, bool, bool)>,
  start: usize,
  end: usize,
}

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
  let input = read_input_string("day12.txt")?;

  let graph = create_graph(input);

  let size = graph.width * graph.height;

  let mut queue = VecDeque::<usize>::with_capacity(size);
  queue.push_back(graph.start);

  let mut distance = vec![i32::MAX; size];
  distance[graph.start] = 0;

  let part1 = dijkstra2(&graph, &mut queue, &mut distance);

  queue.clear();
  
  let mut distance = vec![i32::MAX; size];

  graph.vertices
    .iter()
    .enumerate()
    .filter(|(_, c)| **c == b'a')
    .for_each(|(start, _)| {
      queue.push_back(start);
      distance[start] = 0;
    });

  let part2 = dijkstra2(&graph, &mut queue, &mut distance);

  Ok((part1.to_string(), part2.to_string()))
}

fn dijkstra2(graph: &Graph, queue: &mut VecDeque<usize>, distance: &mut Vec<i32>) -> i32 {
  while !queue.is_empty() {
    let u = queue.pop_front().unwrap();
    
    let (top, right, bottom, left) = graph.edges[u];

    if top {
      let v = u - graph.width;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        queue.push_back(v);
      }
    }

    if right {
      let v = u + 1;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        queue.push_back(v);
      }
    }

    if bottom {
      let v = u + graph.width;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        queue.push_back(v);
      }
    }

    if left {
      let v = u - 1;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        queue.push_back(v);
      }
    }
  }

  distance[graph.end]
}

#[allow(dead_code)]
fn dijkstra<const PRINT: bool>(graph: &Graph) -> i32 {
  let size = graph.width * graph.height;

  let mut queue = VecDeque::<usize>::with_capacity(size);
  queue.push_back(graph.start);

  let mut distance = vec![i32::MAX; size];
  distance[graph.start] = 0;

  let mut previous = vec![None; size];
  
  while !queue.is_empty() {
    let u = queue.pop_front().unwrap();

    let (top, right, bottom, left) = graph.edges[u];

    if top {
      let v = u - graph.width;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        previous[v] = Some(u);
        queue.push_back(v);
      }
    }

    if right {
      let v = u + 1;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        previous[v] = Some(u);
        queue.push_back(v);
      }
    }

    if bottom {
      let v = u + graph.width;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        previous[v] = Some(u);
        queue.push_back(v);
      }
    }

    if left {
      let v = u - 1;
      let alt = distance[u] + 1;
      if alt < distance[v] {
        distance[v] = alt;
        previous[v] = Some(u);
        queue.push_back(v);
      }
    }
  }

  if PRINT {
    print_path(&previous, &graph);
  }

  distance[graph.end]
}

fn print_path(previous: &Vec<Option<usize>>, graph: &Graph) {
  let mut output = vec![b'.'; graph.width * graph.height];
  
  output[graph.end] = b'E';
  
  let mut current = graph.end;
  while let Some(index) = previous[current] {
    output[index] = b'#';

    if index == current + graph.width{
      output[index] = b'^';
    } else if index as i32 == current as i32 - 1 {
      output[index] = b'>';
    } else if index as i32 == current as i32 - graph.width as i32 {
      output[index] = b'V';
    } else if index == current + 1 {
      output[index] = b'<';
    }

    current = index;
  }
  
  output[current] = b'S';

  (0..=graph.height).for_each(|i| output.insert(i * (graph.width + 1), b'\n'));


  println!("{}", String::from_utf8(output).unwrap());
}

fn create_graph(input: String) -> Graph {
  let width = input.find('\n').unwrap();

  let mut vertices = input
    .into_bytes()
    .into_iter()
    .filter(|c| *c != b'\n')
    .collect::<Vec<_>>();

  let height = vertices.len() / width;
  let size = height * width;

  let mut edges = Vec::with_capacity(size);
  let mut start = 0;
  let mut end = 0; 

  for i in 0..size {
    let current = vertices[i];
    if current == b'S' {
      start = i; 
      vertices[i] = b'a';
    } else if current == b'E' {
      end = i;
      vertices[i] = b'z';
    }

    edges.push(get_edges(&vertices, width, height, i));
  }

  edges[end - width] = get_edges(&vertices, width, height, end - width);
  edges[end - 1] = get_edges(&vertices, width, height, end - 1);

  Graph {
    width,
    height,
    vertices,
    edges,
    start,
    end,
  }
}

fn get_edges(vertices: &Vec<u8>, width: usize, height: usize, index: usize) -> (bool, bool, bool, bool) {
  let mut ret = (false, false, false, false);

  let vertex = vertices[index];

  if index >= width {
    let top = vertices[index - width];

    if vertex > top - 2 {
      ret.0 = true;
    }
  }

  if index % width < width - 1 {
    let right = vertices[index + 1];
    
    if vertex > right - 2 {
      ret.1 = true;
    }
  }

  if index / width < height - 1 {
    let bottom = vertices[index + width];
    if vertex > bottom - 2 {
      ret.2 = true;
    }
  }

  if index % width > 0 {
    let left = vertices[index - 1];
    
    if vertex > left - 2 {
      ret.3 = true;
    }
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::{get_edges, Graph, dijkstra};

  #[test]
  fn test_dijkstra() {
    let input = Graph {
      width: 8,
      height: 5,
      vertices: vec![
        b'a', b'a', b'b', b'q', b'p', b'o', b'n', b'm',
        b'a', b'b', b'c', b'r', b'y', b'x', b'x', b'l',
        b'a', b'c', b'c', b's', b'z', b'z', b'x', b'k',
        b'a', b'c', b'c', b't', b'u', b'v', b'w', b'j',
        b'a', b'b', b'd', b'e', b'f', b'g', b'h', b'i',
      ],
      edges: vec![
        (false, true, true, false), (false, true, true, true), (false, false, true, true), (false, true, true, true), (false, true, false, true), (false, true, false, true), (false, true, false, true), (false, false, true, true),
        (true, true, true, false), (true, true, true, true), (true, false, true, true), (true, false, true, true), (true, true, true, true), (true, true, false, true), (true, true, true, true), (true, false, true, false), 
        (true, false, true, false), (true, true, true, true), (true, false, true, true), (true, false, true, true), (true, true, true, true), (true, true, true, true), (true, true, true, false), (true, false, true, false),
        (true, false, true, false), (true, true, true, true), (true, false, true, true), (true, true, true, true), (false, true, true, true), (false, true, true, true), (true, true, true, true), (true, false, true, false),
        (true, true, false, false), (true, false, false, true), (true, true, false, true), (false, true, false, true), (false, true, false, true), (false, true, false, true), (false, true, false, true), (true, false, false, true),
      ],
      start: 0,
      end: 21,
    };

    assert_eq!(dijkstra::<false>(&input), 31);
  }
  
  #[test]
  fn test_get_edges() {
    let input = vec![
      b'a', b'a', b'b', b'q', b'p', b'o', b'n', b'm',
      b'a', b'b', b'c', b'r', b'y', b'x', b'x', b'l',
      b'a', b'c', b'c', b's', b'z', b'z', b'x', b'k',
      b'a', b'c', b'c', b't', b'u', b'v', b'w', b'j',
      b'a', b'b', b'd', b'e', b'f', b'g', b'h', b'i',
    ];

    (0..40).for_each(|i| {
      let (top, right, bottom, left) = get_edges(&input, 8, 5, i);

      if i / 8 == 0 {
        assert!(!top);
      }

      if i % 8 == 7 {
        assert!(!right);
      }

      if i / 8 == 4 {
        assert!(!bottom);
      }

      if i % 8 == 0 {
        assert!(!left);
      }

      let current = input[i];

      if top {
        let top = input[i - 8];
        assert!(current >= top - 1);
      }

      if right {
        let right = input[i + 1];
        assert!(current >= right - 1);
      }

      if bottom {
        let bottom = input[i + 8];
        assert!(current >= bottom - 1);
      }

      if left {
        let left = input[i - 1];
        assert!(current >= left - 1);
      }
    })
  }
}