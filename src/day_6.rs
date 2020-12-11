#![feature(destructuring_assignment)]
use std::fs;
use std::collections::btree_map::BTreeMap;

fn main() {
    println!("Hello, world!");
    println!("Part 1");
    day_6(false);
    println!("Part 2");
    day_6(true);
}

fn process_questionaire(questionaire: &str, everyone:bool) -> usize {
  let mut count = BTreeMap::new();
  let mut comma_count = 0;
  // println!("process questionaire\n[{}]", questionaire);

  for c in questionaire.chars() {
    if c.is_alphabetic() {
      *count.entry(c).or_insert(0) += 1;
    } else if c == ',' {
      comma_count += 1;
    }
  }
  if !everyone {
    return count.len();
  }
  if comma_count > 1 {
    comma_count -= 1; // stupid fix but w/e
  }
  let mut everyone_count = 0;
  let total_count = count.len();
  for (ch, count) in &count {
    println!("{:?}: {}", ch, count);
    if *count == comma_count {
      everyone_count += 1;
    }
  }
  println!("everyone count {} / {} comma count {}", everyone_count, total_count, comma_count);
  return everyone_count;
}

fn day_6(validate:bool) {
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day6.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  let lines: Vec<&str> = contents
    .lines()
    .collect();
  let mut group = "".to_owned();
  let mut count = vec![];
  for (i, line) in lines.iter().enumerate() {
    group.push_str(line);
    if group.len() > 0 {
      group.push_str(",");
    }
    // println!("line {} / {}", i, lines.len());
    if line.len() == 0 || i == lines.len() - 1 {
      count.push(process_questionaire(&group, validate));
      // total += 1;
      // if group.len() > 0 && process_questionaire(&group, validate) == true {
      //   total_valid += 1;
      // }
      group = "".to_owned();
    }
  }
  let total:usize = count.iter().sum();
  println!("total {}", total); // 3213
 
}

