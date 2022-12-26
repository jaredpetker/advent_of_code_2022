use std::collections::{HashMap, HashSet, VecDeque};

mod dir {
  pub static N: (i32, i32) = (0i32, -1i32);
  pub static W: (i32, i32) = (-1i32, 0i32);
  pub static E: (i32, i32) = (1i32, 0i32);
  pub static S: (i32, i32) = (0i32, 1i32);
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Point(i32, i32);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct SeenHash(Point, i32);

impl Point {
  fn add(&self, p: &Self) -> Self {
    Point(self.0 + p.0, self.1 + p.1)
  }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum Env {
  Wall,
  Blizzard((i32, i32)),
}

impl From<char> for Env {
  fn from(c: char) -> Self {
    match c {
      '#' => Env::Wall,
      '>' => Env::Blizzard(dir::E),
      '<' => Env::Blizzard(dir::W),
      '^' => Env::Blizzard(dir::N),
      'v' => Env::Blizzard(dir::S),
      _ => panic!("bad env input")
    }
  }
}

struct Valley {
  width: i32,
  height: i32,
  start: Point,
  end: Point,
  expedition: Point,
  walls: Vec<Point>,
  blizzard: Vec<(Env, Point)>,
  blizzard_at_time: HashMap<i32, HashSet<Point>>,
}

impl From<&str> for Valley {
  fn from(input: &str) -> Self {
    let mut width = 0;
    let mut height = 0;
    let mut blizzard: Vec<(Env, Point)> = Vec::new();
    let mut walls = Vec::new();
    let mut first_path: Option<Point> = None;
    let mut last_path: Option<Point> = None;
    for (y, line) in input.lines().enumerate() {
      height += 1;
      width = line.len() as i32;
      for (x, c) in line.chars().enumerate() {
        let p = Point(x as i32, y as i32);
        match c {
          '#' => { walls.push(p); }
          '.' => {
            if first_path.is_none() {
              first_path = Some(p);
            }
            last_path = Some(p.clone());
            continue;
          }
          b => { blizzard.push((b.into(), p)); }
        }
      }
    }
    Valley {
      walls,
      blizzard,
      width,
      height,
      start: first_path.unwrap(),
      end: last_path.unwrap(),
      expedition: first_path.unwrap().clone(),
      blizzard_at_time: HashMap::new(),
    }
  }
}

impl Valley {
  fn get_current_blizzard(&mut self, minute: i32) -> HashSet<Point> {
    return self.blizzard_at_time.entry(minute)
      .or_insert_with(|| {
        let w = self.width - 2;
        let h = self.height - 2;
        let blizzard_iter = self.blizzard.iter().enumerate().map(|(_, (b, pos))| {
          if let Env::Blizzard(dir) = b {
            Point(
              if dir.0 == 0 { pos.0 } else { ((pos.0 - 1 + dir.0 * minute).rem_euclid(w)) + 1 },
              if dir.1 == 0 { pos.1 } else { ((pos.1 - 1 + dir.1 * minute).rem_euclid(h)) + 1 },
            )
          } else {
            panic!("Non-blizzard in the blizzard!")
          }
        });
        return HashSet::from_iter(blizzard_iter);
      }).to_owned();
  }

  fn find_end(&mut self, from_step: i32) -> i32 {
    let dirs = vec![dir::N, dir::W, dir::E, dir::S];
    let mut moves = VecDeque::from([(self.expedition.clone(), from_step + 1)]);
    let mut seen_cache = HashSet::new();
    while !moves.is_empty() {
      let (next, step) = moves.pop_front().unwrap();
      let blizzard = self.get_current_blizzard(step);

      // cache seen positions / times
      if seen_cache.contains(&SeenHash(next.clone(), step.clone())) {
        continue;
      }
      seen_cache.insert(SeenHash(next, step));

      for dir in dirs.iter() {
        let test = next.add(&Point(dir.0, dir.1));
        if test == self.end {
          self.expedition = test.clone();
          return step;
        } else if !blizzard.contains(&test) && !self.walls.contains(&test) && test.1 > -1 && test.1 < self.height {
          // empty spot
          moves.push_back((test, step + 1));
        }
      }

      if !blizzard.contains(&next) {
        moves.push_back((next, step + 1));
      }
    }
    panic!("failed to find the end!")
  }
}

fn main() {
  let input = include_str!("../input.txt");
  let mut valley = Valley::from(input);

  // to the end
  let start = valley.start.clone();
  let end = valley.end.clone();
  let first = valley.find_end(0);

  // back to the start
  valley.start = end.clone();
  valley.end = start.clone();
  let second = valley.find_end(first);

  // and the end once more!
  valley.start = start.clone();
  valley.end = end.clone();
  let third = valley.find_end(second);
  println!("{}", third);
}
