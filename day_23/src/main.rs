extern crate core;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Point(i32, i32);

fn main() {
  let input = include_str!("../input.txt");
  let dir_list = vec![
    (vec![Point(0, -1), Point(1, -1), Point(-1, -1)], Point(0, -1)),
    (vec![Point(0, 1), Point(1, 1), Point(-1, 1)], Point(0, 1)),
    (vec![Point(-1, 0), Point(-1, 1), Point(-1, -1)], Point(-1, 0)),
    (vec![Point(1, 0), Point(1, -1), Point(1, 1)], Point(1, 0)),
  ];

  let mut elves = HashSet::<Point>::from_iter(
    input.lines().enumerate().flat_map(|(y, line)| {
      line.chars().enumerate().filter_map(move |(x, c)| {
        if c == '#' {
          Some(Point(x as i32, y as i32))
        } else {
          None
        }
      })
    })
  );

  let rounds = usize::MAX;
  let part_1_round = 10;
  let mut proposals: HashMap<Point, Vec<Point>> = HashMap::new();
  for round in 0..rounds {
    let dirs_list_start_i: usize = round % dir_list.len();
    for elf in elves.iter() {

      let mut has_neighbor = false;
      'ring: for x in -1..=1 {
        for y in -1..=1 {
          if !(x == 0 && y == 0) {
            if elves.contains(&Point(elf.0 + x, elf.1 + y)) {
              has_neighbor = true;
              break 'ring;
            }
          }
        }
      }

      if !has_neighbor == false {
        for dirs_list_i in dirs_list_start_i..(dirs_list_start_i + dir_list.len()) {
          let mapped_i: usize = dirs_list_i % dir_list.len();
          let (dirs, dir) = &dir_list[mapped_i];
          let count: i32 = dirs.iter().filter(|d| {
            !elves.contains(&Point(elf.0 + d.0, elf.1 + d.1))
          }).count() as i32;
          if count == 3 {
            let next = Point(elf.0 + dir.0, elf.1 + dir.1);
            proposals.entry(next)
              .and_modify(|v| { v.push(elf.clone()) })
              .or_insert(vec![elf.clone()]);
            break;
          }
        }
      }

    }

    let mut modified = false;
    proposals.drain().for_each(|(p, v)| {
      if v.len() == 1 {
        modified = true;
        elves.remove(&v[0]);
        elves.insert(p);
      }
    });

    if round == part_1_round - 1 {
      // find the bounding box
      let mut min_x = i32::MAX;
      let mut max_x = i32::MIN;
      let mut min_y = i32::MAX;
      let mut max_y = i32::MIN;
      elves.iter().for_each(|elf| {
        min_x = min_x.min(elf.0);
        max_x = max_x.max(elf.0);
        min_y = min_y.min(elf.1);
        max_y = max_y.max(elf.1);
      });
      println!("part 1 answer: {}", (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32);
    }

    if !modified {
      println!("part 2 answer: {}", round + 1);
      break;
    }
  }
}
