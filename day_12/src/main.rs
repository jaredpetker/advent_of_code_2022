use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

fn main() {
  let input = include_str!("../input.txt");
  let split = input.split("\n");
  let mut start = Pos(0, 0);
  let mut end = Pos(0, 0);
  let v: Vec<Vec<char>> = split.enumerate().map(|(r, row)| {
    row.chars().enumerate().map(|(c, chr)| {
      let mut mapped = chr;
      if chr == 'S' {
        mapped = 'a';
        start = Pos(r as i32, c as i32);
      } else if chr == 'E' {
        mapped = 'z';
        end = Pos(r as i32, c as i32);
      }
      return mapped;
    }).collect()
  }).collect();

  let mut set: HashSet<Pos> = HashSet::new();
  let mut queue: VecDeque<(i32, Pos)> = VecDeque::new();
  queue.push_back((0, end.clone()));
  set.insert(end.clone());
  let mut part_2 = 0;
  let mut part_1 = 0;
  while queue.len() > 0 {
    let (dist, curr) = queue.pop_front().unwrap();
    let dirs = vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1), (curr.0, curr.1 + 1), (curr.0, curr.1 - 1)];
    for (row, col) in dirs {
      if (row >= 0 && row < v.len() as i32 && col >= 0 && col < v[0].len() as i32) &&
        !set.contains(&Pos(row, col)) &&
        v[row as usize][col as usize] as i32 - v[curr.0 as usize][curr.1 as usize] as i32 >= -1 {
        if v[row as usize][col as usize] == 'a' && part_2 == 0 {
          part_2 = dist + 1;
        }
        if row == start.0 && col == start.1 {
          part_1 = dist + 1;
          queue.clear();
          break;
        }
        set.insert(Pos(row, col));
        queue.push_back((dist + 1, Pos(row, col)));
      }
    }
  }
  println!("part 1 answer: {}", part_1);
  println!("part 2 answer: {}", part_2);
}

