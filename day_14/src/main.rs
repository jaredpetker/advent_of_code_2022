use std::cmp::Ordering;
use std::collections::HashMap;
use std::slice;
use regex::{Regex, Captures};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Point {
  Rock,
  Sand,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Outcome {
  RockOrSand,
  Floor,
  Blocked
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
  fn down(&self) -> Pos {
    Pos(self.0, self.1 + 1)
  }
  fn up(&self) -> Pos {
    Pos(self.0, self.1 + 1)
  }
  fn left(&self) -> Pos {
    Pos(self.0 - 1, self.1)
  }
  fn right(&self) -> Pos {
    Pos(self.0 + 1, self.1)
  }
}

impl<'t> From<Captures<'t>> for Pos {
  fn from(captures: Captures) -> Self {
    Pos(captures[1].parse().unwrap(), captures[2].parse().unwrap())
  }
}

#[derive(Clone, Debug)]
struct Cave {
  point_map: HashMap<Pos, Point>,
  num_sand_dropped: i32,
  lowest_rock_y: i32,
  sand_drop_pos: Pos
}

impl Cave {
  fn new() -> Cave {
    Cave {
      point_map: HashMap::new(),
      num_sand_dropped: 0,
      lowest_rock_y: 0,
      sand_drop_pos: Pos(500, 0)
    }
  }

  fn parse(input: &str) -> Cave {
    let mut cave = Cave::new();
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    input.lines().for_each(|line| {
      re.captures_iter(line)
        .map(|captures| {
          let pos: Pos = captures.into();
          if pos.1 > cave.lowest_rock_y {
            cave.lowest_rock_y = pos.1;
          }
          return pos;
        })
        .collect::<Vec<Pos>>()
        .windows(2)
        .for_each(|endpoints| {
          cave.scan_rocks(&endpoints[0], &endpoints[1]);
        });
    });
    return cave;
  }

  fn scan_rocks(&mut self, from: &Pos, to: &Pos) {
    let mut xs = vec![from.0, to.0];
    let mut ys = vec![from.1, to.1];

    xs.sort();
    ys.sort();

    for x in xs[0]..xs[1] + 1 {
      for y in ys[0]..ys[1] + 1 {
        self.point_map.insert(Pos(x, y), Point::Rock);
      }
    }
  }

  fn settle_sand(&mut self, sand_pos: &Pos) -> Outcome {
    return if self.point_map.get(&sand_pos).is_some() && *sand_pos == self.sand_drop_pos {
      Outcome::Blocked
    } else if sand_pos.1 == self.lowest_rock_y + 1 {
      self.point_map.insert(sand_pos.clone(), Point::Sand);
      Outcome::Floor
    } else {
      let mid = sand_pos.down();
      let left = mid.left();
      let right = mid.right();
      match (self.point_map.get(&left), self.point_map.get(&mid), self.point_map.get(&right)) {
        (_,None,_) => {
          self.settle_sand(&mid)
        }
        (None, _, _) => {
          self.settle_sand(&left)
        }
        (_, _, None) => {
          self.settle_sand(&right)
        }
        _ => {
          self.point_map.insert(sand_pos.clone(), Point::Sand);
          Outcome::RockOrSand
        }
      }
    }
  }

  fn drop_sand(&mut self) -> Outcome {
    self.settle_sand(&self.sand_drop_pos.clone())
  }

}

fn main() {
  let input = include_str!("../input.txt");
  let mut cave = Cave::parse(input);

  let mut settled_sand = 0;
  let mut floor_hit = false;

  loop {
    let outcome = cave.drop_sand();
    if outcome == Outcome::Floor && !floor_hit {
      floor_hit = true;
      println!("part 1 answer: {}", settled_sand);
    }
    if outcome == Outcome::Blocked {
      println!("part 2 answer: {}", settled_sand);
      break
    }
    settled_sand += 1;
  }
}