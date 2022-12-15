use std::collections::{HashMap, HashSet};
use regex::{Regex, Captures};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Equipment(Pos, Pos, i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

impl Pos {
  fn manhattan_distance_to(&self, pos: &Pos) -> i64 {
    (self.0 - pos.0).abs() + (self.1 - pos.1).abs()
  }
}

impl<'t> From<Option<Captures<'t>>> for Pos {
  fn from(option: Option<Captures>) -> Self {
    let captures = option.unwrap();
    Pos(captures[1].parse().unwrap(), captures[2].parse().unwrap())
  }
}

#[derive(Clone, Debug)]
struct Range {
  left: i64,
  right: i64,
}

impl Range {
  fn new(left: i64, right: i64) -> Range {
    Range {
      left,
      right
    }
  }

  fn overlap(&self, range: &Range) -> bool {
    (range.left >= self.left && range.left <= self.right)
      || (range.right >= self.left && range.right <= self.right)
      || (range.left <= self.left && range.right >= self.right)
      || (self.left <= range.left && self.right >= range.right)
      || (self.right == range.left - 1)
      || (self.left == range.right + 1)

  }

  fn combine_with(&mut self, range: &Range) {
    self.left = self.left.min(range.left);
    self.right = self.right.max(range.right);
  }
}

trait InsertSortedCombined<T> {
  fn insert_sorted_and_combined(&mut self, item: T);
}

impl InsertSortedCombined<Range> for Vec<Range> {
  fn insert_sorted_and_combined(&mut self, item: Range) {
    let mut range_to_carry: Option<usize> = None;
    let mut inserted = false;
    for i in (0..self.len()).rev() {
      if let Some(i_carried) = range_to_carry {
        if self[i].overlap(&self[i_carried])  {
          let carried = self[i_carried].clone();
          self[i].combine_with(&carried);
          self.remove(i_carried);
          range_to_carry = Some(i);
        } else {
          return;
        }
      } else if self[i].overlap(&item) {
        self[i].combine_with(&item);
        range_to_carry = Some(i);
        inserted = true;
      } else if self[i].left > item.left && (i == 0 || self[i - 1].left < item.left) {
        self.insert( i, item.clone());
        inserted = true;
        return;
      }
    }
    if !inserted {
      self.push(item)
    }
  }
}

#[derive(Clone, Debug)]
struct Tunnels {
  just_beacons: HashSet<Pos>,
  equipment: Vec<Equipment>
}

impl Tunnels {
  fn new() -> Tunnels {
    Tunnels {
      equipment: vec![],
      just_beacons: HashSet:: new()
    }
  }

  fn parse(input: &str) -> Tunnels {
    let mut tunnels = Tunnels::new();
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    input.lines().for_each(|line| {
      let mut iter = re.captures_iter(line);
      let sensor_pos: Pos = iter.next().into();
      let beacon_pos: Pos = iter.next().into();
      tunnels.just_beacons.insert(beacon_pos.clone());
      let manhattan_distance = sensor_pos.manhattan_distance_to(&beacon_pos);
      tunnels.equipment.push(Equipment(sensor_pos, beacon_pos, manhattan_distance));
    });
    return tunnels;
  }

  fn get_unbeaconable_ranges_for_row(&mut self, row: i64) -> Vec<Range> {
    let mut left = i64::MAX;
    let mut right = i64::MIN;
    let mut ranges: Vec<Range> = vec![];
    for equip in self.equipment.iter() {
      let Equipment(Pos(sx, sy), _, manhattan_distance) = equip;
      let dy = (sy - row).abs();
      if dy <= *manhattan_distance {
        let dx = manhattan_distance - dy;
        let left = (sx - dx).min(left);
        let right = (sx + dx).max(right);
        ranges.insert_sorted_and_combined(Range{left, right});
      }
    }
    return ranges;
  }

}

fn main() {
  let input = include_str!("../input.txt");
  let mut tunnels = Tunnels::parse(input);
  let part_1_row = 2000000;
  let max_rows = 4000000;
  let mut part_1_answer: Option<i64> = None;
  let mut part_2_answer: Option<i64> = None;
  for row in 0..max_rows {
    let ranges = tunnels.get_unbeaconable_ranges_for_row(row);
    if row == part_1_row {
      let beacons_y_count = tunnels.just_beacons.iter().filter(|Pos(_, by)| {*by == row }).count() as i64;
      part_1_answer = Some(1 + ranges[0].right - ranges[0].left - beacons_y_count);
    }
    if ranges.len() == 2 {
      part_2_answer = Some((ranges[0].right + 1)*4000000 + row);
    }
    if part_1_answer.is_some() && part_2_answer.is_some() {
      break
    }
  }
  println!("part 1 answer: {}", part_1_answer.unwrap());
  println!("part 2 answer: {}", part_2_answer.unwrap());
}