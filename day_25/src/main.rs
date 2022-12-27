#[derive(Clone, Debug, Eq, PartialEq)]
struct Snafu(String);

impl From<&str> for Snafu {
  fn from(str: &str) -> Self {
    return Snafu(str.to_owned());
  }
}

impl From<&Snafu> for i64 {
  fn from(snafu: &Snafu) -> Self {
    snafu.0.chars().rev().enumerate().map(|(n, c)| {
      (match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("invalid snafu character")
      }) * 5i64.pow(n as u32)
    }).sum()
  }
}

fn i32_to_snafu_char(i: i32) -> char {
  match i {
    2 => '2',
    1 => '1',
    0 => '0',
    -1 => '-',
    -2 => '=',
    _ => panic!("invalid snafu digit")
  }
}

fn range_at_n(n: u32) -> (i64, i64, i64) {
  let last_range = if n > 0 { range_at_n(n - 1) } else { (0, 0, 0) };
  let e = 5i64.pow(n);
  let lo = e - last_range.2;
  let hi = e * 2 + last_range.2;
  return (lo, e, hi);
}

fn lo_range_at_n(n: u32) -> (i64, i64, i64) {
  let last_range = if n > 0 { range_at_n(n - 1) } else { (0, 0, 0) };
  let e = 5i64.pow(n);
  let lo = e - last_range.2;
  let hi = e + last_range.2;
  return (lo, e, hi);
}

fn hi_range_at_n(n: u32) -> (i64, i64, i64) {
  let last_range = if n > 0 { range_at_n(n - 1) } else { (0, 0, 0) };
  let e = 5i64.pow(n) * 2;
  let lo = e - last_range.2;
  let hi = e + last_range.2;
  return (lo, e, hi);
}

impl From<i64> for Snafu {
  fn from(i: i64) -> Self {
    let abs = i.abs() as i64;
    let mut n = 0;
    loop {
      let range = range_at_n(n);
      if abs > range.1 {
        n += 1;
        continue;
      }
      break;
    }

    let mut str = String::new();
    let mut curr = i;
    let mut hold = 0;
    let mut sign = 1;
    for bit_index in (0..=n).rev() {
      // could cache these
      let lo = lo_range_at_n(bit_index);
      let hi = hi_range_at_n(bit_index);
      let curr_abs = curr.abs() as i64;

      let d = if curr_abs >= lo.0 && curr_abs <= lo.2 {
        hold += (i - hold).signum() * lo.1;
        1 * sign
      } else if curr_abs >= hi.0 && curr_abs <= hi.2 {
        hold += (i - hold).signum() * hi.1;
        2 * sign
      } else {
        0
      };
      str.push(i32_to_snafu_char(d));

      if d != 0 {
        sign = (i - hold).signum() as i32;
        curr = if curr_abs <= lo.2 {
          lo.1
        } else {
          hi.1
        } - curr_abs
      }

    }
    Snafu(str)
  }
}

fn main() {
  let input = include_str!("../input.txt");
  let snafus: Vec<Snafu> = input
    .lines()
    .map(Snafu::from)
    .collect();
  let decimal_sum: i64 = snafus
    .iter()
    .map(|s| <&Snafu as Into<i64>>::into(s))
    .sum();
  println!("{:?}", decimal_sum);
  let snafu: Snafu = decimal_sum.into();
  println!("part 1 answer: {}", snafu.0);
  let from_snafu_check: i64 = (&snafu).into();
  println!("{:?}", from_snafu_check);
}
