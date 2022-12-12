use regex::{Regex, Match};
use std::collections::{VecDeque};

struct U64(u64);
impl <'t> From<Option<Match<'t>>> for U64 {
  fn from(c: Option<Match<'t>>) -> Self {
    U64(c.unwrap().as_str().parse::<u64>().unwrap())
  }
}

struct Usize(usize);
impl <'t> From<Option<Match<'t>>> for Usize {
  fn from(c: Option<Match<'t>>) -> Self {
    Usize(c.unwrap().as_str().parse::<usize>().unwrap())
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OperationOperator {
  Add,
  Mul,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OperationRightValue {
  Int(u64),
  Input,
}

#[derive(Copy, Clone, Debug)]
struct Operation(OperationOperator, OperationRightValue);

enum EvalPostOp {
  Div(u64),
  Mod(u64)
}

#[derive(Clone, Debug)]
struct Monkey {
  starting_items: VecDeque<u64>,
  operation: Operation,
  div_by: u64,
  throw_to_if_true: usize,
  throw_to_if_false: usize,
}

impl Monkey {
  fn test(&self, num: u64, div_by: u64) -> bool {
    return num % div_by == 0u64;
  }

  fn apply_op(&self, old: u64) -> u64 {
    let Operation(operator, value) = self.operation;
    match operator {
      OperationOperator::Add => {
        old + if let OperationRightValue::Int(num) = value { num } else { old }
      }
      OperationOperator::Mul => {
        old * if let OperationRightValue::Int(num) = value { num } else { old }
      }
    }
  }

  fn throw_to(&mut self, item: u64) {
    self.starting_items.push_back(val_to_throw);
  }

  fn eval_items(&mut self, post_op: EvalPostOp) -> Vec<(usize, u64)> {
    let mut items_to_throw: Vec<(usize, u64)> = vec![];
    while self.starting_items.len() > 0 {
      let item = self.starting_items.pop_front().unwrap();
      items_to_throw.push(self.eval(item, &post_op));
    }
    return items_to_throw;
  }

  fn eval(&mut self, item: u64, post_op: &EvalPostOp) -> (usize, u64) {
    let mut new = self.apply_op(item.clone());
    new = match post_op {
      EvalPostOp::Div(num) => new / num,
      EvalPostOp::Mod(num) => new % num,
    };
    let test_output = self.test(new, self.div_by.clone());
    let throw_to = if test_output {
      self.throw_to_if_true
    } else {
      self.throw_to_if_false
    };
    return (throw_to, new);
  }
}

impl From<Vec<&str>> for Monkey {
  fn from(lines: Vec<&str>) -> Self {
    let starting_items_re: Regex = Regex::new(r"(\d+)").unwrap();
    let operation_re: Regex = Regex::new(r"old ([+*]) (\d+|old)").unwrap();
    let test_re: Regex = Regex::new(r"(\d+)").unwrap();
    let items_iter = starting_items_re.captures_iter(lines[1]);
    let starting_items: VecDeque<u64> = items_iter.map(|s| U64::from(s.get(0)).0).collect();
    let op_captures = operation_re.captures(lines[2]).unwrap();
    let op = op_captures.get(1).unwrap().as_str();
    let right_value = op_captures.get(2).unwrap().as_str();
    let operation = Operation(
      if op == "+" { OperationOperator::Add } else { OperationOperator::Mul },
      if right_value == "old" {
        OperationRightValue::Input
      } else {
        OperationRightValue::Int(right_value.parse::<u64>().unwrap())
      },
    );
    let U64(div_by) = test_re.captures(lines[3]).unwrap().get(1).into();
    let Usize(throw_to_if_true) = test_re.captures(lines[4]).unwrap().get(1).into();
    let Usize(throw_to_if_false) = test_re.captures(lines[5]).unwrap().get(1).into();

    return Monkey {
      starting_items,
      operation,
      div_by,
      throw_to_if_true,
      throw_to_if_false,
    }
  }
}

fn main() {
  let input = include_str!("../input.txt");
  let split = input.split("\n\n");
  let mut inspection_counts: Vec<u64> = vec![0; split.clone().count()];

  let monkeys: Vec<Monkey> = split.map(|raw_instruction| {
    let lines: Vec<&str> = raw_instruction.split("\n").collect();
    return Monkey::from(lines);
  }).collect();

  let modulus = monkeys.iter().fold(1, |a, b| { a * b.div_by });

  for part in 1..3 {
    for i in 0..inspection_counts.len() {
      inspection_counts[i] = 0;
    }
    let mut monkeys = monkeys.clone();
    let num_rounds = if part == 1 { 20 } else { 10000 };
    for _ in 0..num_rounds {
      for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let items_to_throw = monkey.eval_items(
          if part == 1 { EvalPostOp::Div(3) } else { EvalPostOp::Mod(modulus) }
        );
        inspection_counts[i] += items_to_throw.len() as u64;
        for (throw_to, val_to_throw) in items_to_throw {
          monkeys[throw_to].throw_to(val_to_throw);
        }
      }
    }
    // for monkey in monkeys.iter_mut() {
    //   println!("{:?}", monkey);
    // }
    inspection_counts.sort();
    inspection_counts.reverse();
    let monkey_business_lvl = inspection_counts[0] * inspection_counts[1];
    println!("part {}: {}", part, monkey_business_lvl)
  }
}
