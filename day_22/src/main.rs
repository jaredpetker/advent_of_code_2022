extern crate core;

use std::collections::HashMap;
use regex::{Regex, Captures};
use crate::Rule::TurnRight;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Vect2D(i32, i32);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Env {
  Wall,
  Path,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Rule {
  Step(i32),
  TurnRight,
  TurnLeft,
}

impl<'t> From<Captures<'t>> for Rule {
  fn from(captures: Captures) -> Self {
    return match &captures[1] {
      "R" => { Rule::TurnRight }
      "L" => { Rule::TurnLeft }
      s => { Rule::Step(s.parse().unwrap()) }
    };
  }
}


struct SideChange {
  to_side: (usize, usize),
  transform: fn((&Vect2D, &Vect2D, i32)) -> (Vect2D, Vect2D)
}



struct JungleCube {
  cube: HashMap<(usize, usize), HashMap<Vect2D, Env>>,
  side_change: HashMap<((usize, usize), Vect2D), SideChange>,
  rules: Vec<Rule>,
  start: ((usize, usize), Vect2D),
}

impl JungleCube {
  fn parse(input: &str) -> JungleCube {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut cube:  HashMap<(usize, usize), HashMap<Vect2D, Env>> = HashMap::new();
    let mut start: ((usize, usize), Vect2D) = ((0, 0), Vect2D(-1, -1));
    for (y, line) in split[0].lines().enumerate() {
      line.chars().enumerate().for_each(|(x, c)| {
        let c_pos = (x / 50, y / 50);
        if c != ' ' {
          if !cube.contains_key(&c_pos) {
            cube.insert(c_pos, HashMap::new());
          }
          let mut m = cube.get_mut(&c_pos).unwrap();
          m.insert(Vect2D(x as i32 % 50, y as i32 % 50), if c == '.' {
            if start.1.0 == -1 {
              start = (c_pos.clone(), Vect2D(x as i32 % 50, y as i32 % 50));
            }
            Env::Path
          } else {
            Env::Wall
          });
        }
      })
    }

    let mut side_change: HashMap<((usize, usize), Vect2D), SideChange> = HashMap::new();

    // 1, 0
    side_change.insert(
      ((1, 0), Vect2D(0, -1)), SideChange
      {
        to_side: (0, 3),
        transform: |(p, d, m)| {
          return (Vect2D(0, p.0), Vect2D(1, 0))
        }
      }
    );
    side_change.insert(
      ((1, 0), Vect2D(1, 0)), SideChange
      {
        to_side: (2, 0),
        transform: |(p, d, m)| {
          (Vect2D(0, p.1), d.clone())
        }
      }
    );
    side_change.insert(
      ((1, 0), Vect2D(0, 1)), SideChange
      {
        to_side: (1, 1),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, 0), d.clone())
        }
      }
    );
    side_change.insert(
      ((1, 0), Vect2D(-1, 0)), SideChange
      {
        to_side: (0, 2),
        transform: |(p, d, m)| {
          return (Vect2D(0, m - p.1), Vect2D(1, 0))
        }
      }
    );
    // GOOd

    // 2, 0
    side_change.insert(
      ((2, 0), Vect2D(0, -1)), SideChange
      {
        to_side: (0, 3),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, m), d.clone())
        }
      }
    );
    side_change.insert(
      ((2, 0), Vect2D(1, 0)), SideChange
      {
        to_side: (1, 2),
        transform: |(p, d, m)| {
          (Vect2D(m, m - p.1), Vect2D(-1, 0))
        }
      }
    );
    side_change.insert(
      ((2, 0), Vect2D(0, 1)), SideChange
      {
        to_side: (1, 1),
        transform: |(p, d, m)| {
          return (Vect2D(m, p.0), Vect2D(-1, 0))
        }
      }
    );
    side_change.insert(
      ((2, 0), Vect2D(-1, 0)), SideChange
      {
        to_side: (1, 0),
        transform: |(p, d, m)| {
          return (Vect2D(m, p.1), d.clone())
        }
      }
    );

    // 1,  1
    side_change.insert(
      ((1, 1), Vect2D(0, -1)), SideChange
      {
        to_side: (1, 0),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, m), d.clone())
        }
      }
    );
    side_change.insert(
      ((1, 1), Vect2D(1, 0)), SideChange
      {
        to_side: (2, 0),
        transform: |(p, d, m)| {
          (Vect2D(p.1, m), Vect2D(0, -1))
        }
      }
    );
    side_change.insert(
      ((1, 1), Vect2D(0, 1)), SideChange
      {
        to_side: (1, 2),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, 0), d.clone())
        }
      }
    );
    side_change.insert(
      ((1, 1), Vect2D(-1, 0)), SideChange
      {
        to_side: (0, 2),
        transform: |(p, d, m)| {
          return (Vect2D(p.1, 0), Vect2D(0, 1))
        }
      }
    );

    // 0,  2
    side_change.insert(
      ((0, 2), Vect2D(0, -1)), SideChange
      {
        to_side: (1, 1),
        transform: |(p, d, m)| {
          return (Vect2D(0, p.0), Vect2D(1, 0))
        }
      }
    );
    side_change.insert(
      ((0, 2), Vect2D(1, 0)), SideChange
      {
        to_side: (1, 2),
        transform: |(p, d, m)| {
          (Vect2D(0, p.1), d.clone())
        }
      }
    );
    side_change.insert(
      ((0, 2), Vect2D(0, 1)), SideChange
      {
        to_side: (0, 3),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, 0), d.clone())
        }
      }
    );
    side_change.insert(
      ((0, 2), Vect2D(-1, 0)), SideChange
      {
        to_side: (1, 0),
        transform: |(p, d, m)| {
          return (Vect2D(0, m - p.1), Vect2D(1, 0))
        }
      }
    );

    // 1,  2
    side_change.insert(
      ((1, 2), Vect2D(0, -1)), SideChange
      {
        to_side: (1, 1),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, m), d.clone())
        }
      }
    );
    side_change.insert(
      ((1, 2), Vect2D(1, 0)), SideChange
      {
        to_side: (2, 0),
        transform: |(p, d, m)| {
          (Vect2D(m, m - p.1), Vect2D(-1, 0))
        }
      }
    );
    side_change.insert(
      ((1, 2), Vect2D(0, 1)), SideChange
      {
        to_side: (0, 3),
        transform: |(p, d, m)| {
          return (Vect2D(m, p.0), Vect2D(-1, 0))
        }
      }
    );
    side_change.insert(
      ((1, 2), Vect2D(-1, 0)), SideChange
      {
        to_side: (0, 2),
        transform: |(p, d, m)| {
          return (Vect2D(m, p.1), d.clone())
        }
      }
    );

    // 0, 3
    side_change.insert(
      ((0, 3), Vect2D(0, -1)), SideChange
      {
        to_side: (0, 2),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, m), d.clone())
        }
      }
    );
    side_change.insert(
      ((0, 3), Vect2D(1, 0)), SideChange
      {
        to_side: (1, 2),
        transform: |(p, d, m)| {
          (Vect2D(p.1, m), Vect2D(0, -1))
        }
      }
    );
    side_change.insert(
      ((0, 3), Vect2D(0, 1)), SideChange
      {
        to_side: (2, 0),
        transform: |(p, d, m)| {
          return (Vect2D(p.0, 0), d.clone())
        }
      }
    );
    side_change.insert(
      ((0, 3), Vect2D(-1, 0)), SideChange
      {
        to_side: (1, 0),
        transform: |(p, d, m)| {
          return (Vect2D(p.1, 0), Vect2D(0, 1))
        }
      }
    );

    // we  need to figure  out a way to stitch this all together easier
    let re = Regex::new(r"(\d+|\w)").unwrap();
    let rules: Vec<Rule> = re.captures_iter(split[1]).map(|captures| {
      let rule: Rule = captures.into();
      rule
    }).collect();

    return JungleCube {
      cube,
      rules,
      start,
      side_change
    };
  }


  fn go(&self) {
    let (mut side, mut pos) = self.start.clone();
    let mut dir = Vect2D(1, 0);
    let left_turns = HashMap::from([
      (Vect2D(0, 1), Vect2D(1, 0)),
      (Vect2D(1, 0), Vect2D(0, -1)),
      (Vect2D(0, -1), Vect2D(-1, 0)),
      (Vect2D(-1, 0), Vect2D(0, 1))
    ]);
    let right_turns = HashMap::from([
      (Vect2D(0, 1), Vect2D(-1, 0)),
      (Vect2D(-1, 0), Vect2D(0, -1)),
      (Vect2D(0, -1), Vect2D(1, 0)),
      (Vect2D(1, 0), Vect2D(0, 1))
    ]);
    let dir_scores = HashMap::from([
      (Vect2D(1, 0), 0),
      (Vect2D(0, 1), 1),
      (Vect2D(-1, 0), 2),
      (Vect2D(0, -1), 3)
    ]);
    for rule in &self.rules {
      if let Rule::Step(steps) = rule {
        for _ in 0..*steps {
          let next = Vect2D(&pos.0 + dir.0, &pos.1 + dir.1);
          let map = &self.cube[&side];
          if !map.contains_key(&next) {
            let side_change = &self.side_change[&(side, dir.clone())];
            let (new_pos, new_dir) = (side_change.transform)((&pos, &dir, 49));
            let new_map = &self.cube[&side_change.to_side];
            if new_map[&new_pos] == Env::Path {
              pos = new_pos.clone();
              dir = new_dir.clone();
              side = side_change.to_side.clone();
            } else {
              break;
            }
          } else if map[&next] == Env::Wall {
            break;
          } else if map[&next] == Env::Path {
            pos = next.clone();
          }
        }
      } else if rule == &Rule::TurnLeft {
        dir = left_turns[&dir].clone()
      } else if rule == &Rule::TurnRight {
        dir = right_turns[&dir].clone()
      }
    }

    let px = (side.0 as i32 * 50) + pos.0 + 1;
    let py = (side.1 as i32 * 50) + pos.1 + 1;
    let f_pos = Vect2D(px, py);
    let dir_score = dir_scores[&dir];
    println!("part 2 answer: {:?}",  1000 * f_pos.1 + 4 * f_pos.0 + dir_score);
  }
}



#[derive(Clone, Debug)]
struct Jungle {
  map: HashMap<Vect2D, Env>,
  rules: Vec<Rule>,
  start: Vect2D,
}

impl Jungle {
  fn parse(input: &str) -> Jungle {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut map: HashMap<Vect2D, Env> = HashMap::new();
    let mut start: Vect2D = Vect2D(-1, -1);
    for (y, line) in split[0].lines().enumerate() {
      line.chars().enumerate().for_each(|(x, c)| {
        if c != ' ' {
          map.insert(Vect2D(x as i32, y as i32), if c == '.' {
            if start.0 == -1 {
              start = Vect2D(x as i32, y as i32);
            }
            Env::Path
          } else {
            Env::Wall
          });
        }
      })
    }
    let re = Regex::new(r"(\d+|\w)").unwrap();
    let rules: Vec<Rule> = re.captures_iter(split[1]).map(|captures| {
      let rule: Rule = captures.into();
      rule
    }).collect();

    return Jungle {
      map,
      rules,
      start,
    };
  }

  fn go(&self) {
    let mut dir = Vect2D(1, 0);
    let mut pos = self.start.clone();
    let left_turns = HashMap::from([
      (Vect2D(0, 1), Vect2D(1, 0)),
      (Vect2D(1, 0), Vect2D(0, -1)),
      (Vect2D(0, -1), Vect2D(-1, 0)),
      (Vect2D(-1, 0), Vect2D(0, 1))
    ]);
    let right_turns = HashMap::from([
      (Vect2D(0, 1), Vect2D(-1, 0)),
      (Vect2D(-1, 0), Vect2D(0, -1)),
      (Vect2D(0, -1), Vect2D(1, 0)),
      (Vect2D(1, 0), Vect2D(0, 1))
    ]);
    let dir_scores = HashMap::from([
      (Vect2D(1, 0), 0),
      (Vect2D(0, 1), 1),
      (Vect2D(-1, 0), 2),
      (Vect2D(0, -1), 3)
    ]);
    for rule in &self.rules {
      if let Rule::Step(steps) = rule {
        for _ in 0..*steps {
          let next = Vect2D(pos.0 + dir.0, pos.1 + dir.1);
          if !self.map.contains_key(&next) {
            // circle around
            // look in the opposite direction
            let opp = Vect2D(dir.0 * -1, dir.1 * -1);
            let mut opp_next = Vect2D(pos.0 + opp.0, pos.1 + opp.1);
            while self.map.contains_key(&opp_next) {
              opp_next = Vect2D(opp_next.0 + opp.0, opp_next.1 + opp.1);
            }
            let check = Vect2D(opp_next.0 + dir.0, opp_next.1 + dir.1);
            if self.map[&check] == Env::Path {
              pos = check.clone();
            }
          } else if self.map[&next] == Env::Wall {
            break;
          } else if self.map[&next] == Env::Path{
            pos = next.clone();
          }
        }
      } else if rule == &Rule::TurnLeft {
        dir = left_turns[&dir].clone()
      } else if rule == &Rule::TurnRight {
        dir = right_turns[&dir].clone()
      }
    }
    let f_pos = Vect2D(pos.0 + 1, pos.1 + 1);
    let dir_score = dir_scores[&dir];
    println!("part 1 answer: {:?}",  1000 * f_pos.1 + 4 * f_pos.0 + dir_score);
  }
}

fn main() {
  let input = include_str!("../input.txt");
  let jungle = Jungle::parse(input);
  jungle.go();

  let jungle_cube = JungleCube::parse(input);
  jungle_cube.go();
}
