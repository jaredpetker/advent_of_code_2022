extern crate core;

use std::collections::HashMap;
use regex::{Regex, Captures};

#[derive(Clone, Debug)]
enum Op {
  Value(f64),
  Mul(String, String),
  Div(String, String),
  Add(String, String),
  Sub(String, String),
}

#[derive(Clone, Debug)]
struct Job(String, Op);

impl<'t> From<Captures<'t>> for Job {
  fn from(captures: Captures) -> Self {
    let op: Op = if captures.get(3).is_none() {
      Op::Value(captures[2].parse().unwrap())
    } else {
      match &captures[4] {
        "+" => { Op::Add(captures[3].to_string(), captures[5].to_string()) }
        "-" => { Op::Sub(captures[3].to_string(), captures[5].to_string()) }
        "*" => { Op::Mul(captures[3].to_string(), captures[5].to_string()) }
        "/" => { Op::Div(captures[3].to_string(), captures[5].to_string()) }
        _ => { panic!("operation unavailable") }
      }
    };
    return Job(captures[1].parse().unwrap(), op);
  }
}

#[derive(Clone, Debug)]
struct Riddle {
  job_map: HashMap<String, Job>,
  job_map_2: HashMap<String, Job>,
}

impl Riddle {
  fn parse(input: &str) -> Riddle {
    let re = Regex::new(r"(\w{4}): (\d+|(\w{4}) ([+\-*/]) (\w{4}))").unwrap();
    let mut job_map: HashMap<String, Job> = HashMap::new();
    let mut job_map_2 = job_map.clone();
    for line in input.lines() {
      let captures = re.captures(line).unwrap();
      let job: Job = captures.into();
      job_map.insert(job.0.clone(), job.clone());
    }
    return Riddle {
      job_map,
      job_map_2
    };
  }

  fn compute_value(&self, name: &str, n: f64) -> f64 {
    if name == "humn" && n > -1f64 {
      return n
    }
    let job = &self.job_map[name];
    return match &job.1 {
      Op::Value(v) => { *v }
      Op::Mul(l, r) => {
        self.compute_value(l.as_str(), n) * self.compute_value(r.as_str(), n)
      }
      Op::Div(l, r) => {
        self.compute_value(l.as_str(), n) / self.compute_value(r.as_str(), n)
      }
      Op::Add(l, r) => {
        self.compute_value(l.as_str(), n) + self.compute_value(r.as_str(), n)
      }
      Op::Sub(l, r) => {
        self.compute_value(l.as_str(), n) - self.compute_value(r.as_str(), n)
      }
      _ => { panic!("unknown op for compute_value") }
    };
  }

}

fn main() {
  let input = include_str!("../input.txt");
  let mut riddle = Riddle::parse(input);
  let v = riddle.compute_value("root", -1f64);
  println!("part 1 answer: {:?}", v as u64);

  let (mut l, mut r) = if let Op::Add(l, r) = &riddle.job_map["root"].1 {
    (l.clone(), r.clone())
  } else {
    panic!("Check the root operation")
  };

  let mut equal_to = r.clone();
  let mut other = l.clone();
  let mut o1 = riddle.compute_value(&equal_to, 0f64);
  let mut o2 = riddle.compute_value(&equal_to, 1000f64);
  if o1 != o2 {
    other = r.clone();
    equal_to = l.clone();
  }
  let val = riddle.compute_value(&equal_to, 0f64);
  let mut min = 0f64;
  let mut max = f64::MAX;
  while min < max {
    let mid: f64 = ((max + min) / 2f64).floor();
    let v = val - riddle.compute_value(&other, mid);
    if v < 0f64 {
      min = mid
    } else if v > 0f64 {
      max = mid
    } else {
      println!("part 2 answer: {}", mid as u64);
      return;
    }
  }
}
