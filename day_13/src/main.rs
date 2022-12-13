use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Packet {
  List(Vec<Packet>),
  Int(i32)
}

fn ordering(order: i32) -> Ordering {
  return if order < 0 {
    Ordering::Less
  } else if order == 0 {
    Ordering::Equal
  } else {
    Ordering::Greater
  }
}

fn parse_packet(input: &str) -> Packet {
  let mut queue: Vec<Vec<Packet>> = vec!();
  let mut digit: String = String::new();
  for c in input.chars() {

    if !c.is_digit(10) && digit.len() > 0 {
      queue.last_mut().unwrap().push(Packet::Int(digit.parse::<i32>().unwrap()));
      digit.clear();
    }

    if c == '[' {
      queue.push(vec!());
    } else if c == ']' {
      let list = queue.pop().unwrap();
      if queue.len() == 0 {
        return Packet::List(list);
      } else {
        queue.last_mut().unwrap().push(Packet::List(list));
      }
    } else if c.is_digit(10) {
      digit.push(c as char);
    }

  }
  return Packet::List(vec!());
}

fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
  return match (left, right) {
    (Packet::List(left_list), Packet::List(right_list)) => {
      for li in 0..left_list.len() {
        if li + 1 > right_list.len() {
          break
        }
        let order = compare_packets(&left_list[li], &right_list[li]);
        if order == Ordering::Equal {
          continue
        } else {
          return order
        }
      }
      ordering(right_list.len() as i32 - left_list.len() as i32)
    },
    (Packet::List(_), Packet::Int(right_int)) => {
      compare_packets(left, &Packet::List(vec![Packet::Int(*right_int)]))
    },
    (Packet::Int(left_int), Packet::List(_)) => {
      compare_packets(&Packet::List(vec![Packet::Int(*left_int)]), right)
    },
    (Packet::Int(left_int), Packet::Int(right_int)) => {
      ordering(right_int - left_int)
    }
  }
}

fn main() {
  let input = include_str!("../input.txt");
  let pairs = input.split("\n\n");
  let mut packets: Vec<Packet> = vec!();
  let mut sum = 0;
  for (i, pair) in pairs.enumerate() {
    let mut lists = pair.split("\n");
    let left = lists.next().unwrap();
    let right = lists.next().unwrap();
    let left_packet = parse_packet(left);
    let right_packet = parse_packet(right);
    let order = compare_packets(&left_packet, &right_packet);
    if order == Ordering::Greater {
      sum += i + 1;
    }
    packets.push(left_packet);
    packets.push(right_packet);
  }

  println!("part 1 answer: {}", sum);

  let decoder_key_1 = parse_packet("[[2]]");
  let decoder_key_2 = parse_packet("[[6]]");
  packets.push(decoder_key_1.clone());
  packets.push(decoder_key_2.clone());
  packets.sort_by(compare_packets);
  let decoder_key_1_pos = packets.len() - packets.iter().position(|a| *a == decoder_key_1).unwrap();
  let decoder_key_2_pos = packets.len() - packets.iter().position(|a| *a == decoder_key_2).unwrap();

  println!("part 2 answer: {}", decoder_key_1_pos * decoder_key_2_pos);
}

