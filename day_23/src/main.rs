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
  let mut set = HashSet::<Point>::new();
  input.lines().enumerate().for_each(|(y, line)| {
    line.chars().enumerate().for_each(|(x, c)| {
      if c == '#' {
        set.insert(Point(x as i32, y as i32));
      }
    });
  });

  let rounds = usize::MAX;
  let part_1_round = 10;
  let mut proposals = HashMap::<Point, Vec<Point>>::new();
  for round in 0..rounds {
    let dir_start_i: usize = round % dir_list.len();
    for elf in set.iter() {

      let mut found = false;
      'ring: for x in -1..=1 {
        for y in -1..=1 {
          if !(x == 0 && y == 0) {
            if set.contains(&Point(elf.0 + x, elf.1 + y)) {
              found = true;
              break 'ring;
            }
          }
        }
      }

      if found == false {
        continue
      }

      for di in dir_start_i..(dir_start_i + dir_list.len()) {
        let dir_i: usize = di % dir_list.len();
        let (dirs, dir) = &dir_list[dir_i];
        let sum: i32 = dirs.iter().map(|d| {
          let next = Point(elf.0 + d.0, elf.1 + d.1);
          if !set.contains(&next) { 1 } else { 0 }
        }).sum();
        if sum == 3 {
          let next = Point(elf.0 + dir.0, elf.1 + dir.1);
          proposals.entry(next)
            .and_modify(|v| {v.push(elf.clone())})
            .or_insert(vec![elf.clone()]);
          break
        }
      }
    }
    let mut modified = false;
    proposals.iter().filter(|(_, v)| {
      v.len() == 1
    }).for_each(|(p, v)| {
      modified = true;
      set.remove(&v[0]);
      set.insert(*p);
    });
    proposals.clear();

    if round == part_1_round - 1 {
      // find the bounding box
      let mut min_x = i32::MAX;
      let mut max_x = i32::MIN;
      let mut min_y = i32::MAX;
      let mut max_y = i32::MIN;
      set.iter().for_each(|elf| {
        min_x = min_x.min(elf.0);
        max_x = max_x.max(elf.0);
        min_y = min_y.min(elf.1);
        max_y = max_y.max(elf.1);
      });
      println!("part 1 answer: {}", (max_x - min_x + 1) * (max_y - min_y + 1) - set.len() as i32);
    }

    if !modified {
      println!("part 2 answer: {}", round + 1);
      break
    }

  }

}
