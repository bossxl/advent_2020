#![feature(destructuring_assignment)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

fn main() {
    println!("Hello, world!");
    // println!("Part 1");
    // day_8(false);
    println!("Part 2");
    day_8(true);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .collect()
}

fn create_command(cmd:&String) -> (&str, i32, i32) {
  let command_parts: Vec<&str> = cmd.split_whitespace().collect();
  let direction:i32;
  if command_parts[1].contains("+") {
    direction = 1;
  } else {
    direction = -1;
  }
  let sNumber = command_parts[1].replace("+", "").replace("-", "");

  return (command_parts[0],direction,sNumber.parse::<i32>().unwrap());
}

fn day_8(mod_check: bool) {
  // let filename = "/Users/pboss/workspace/advent/Y2020/data/day_8-example.txt";
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day_8.txt";
  let data = lines_from_file(filename);
  let mut done = false;
  let mut looped = false;
  let mut error = false;
  let oob = usize::MAX;
  let mut super_count = 0;

  let mut modded = false;
  let mut modded_run = false;
  let mut trace: Vec<i32> = Vec::new();
  let mut line_number = 0;
  let mut acc = 0;
  let mut count = 0;
  
  let max = data.len();

  while !looped && !done && !error {
    count += 1;
    let mut cmd: (&str, i32, i32) = ("err", 0, 0);
    match data.get(line_number as usize) {
      Some(line) => {
        cmd = create_command(line);
      },
      None =>  { 
        done = true
      },
    }
    trace.push(line_number);
    if !done {
      if mod_check {
        if line_number == super_count {
          modded = true;
          println!("mod count {}", super_count);
        }
      }
      match cmd.0 {
        "acc" => { 
          acc += cmd.1 * cmd.2;
          line_number += 1;
          // println!("acc {} | {}", cmd.1 * cmd.2, count);
        }
        "jmp" => { 
          if modded && !modded_run {
            modded_run = true;
            line_number += 1;
            // println!("nop {} | {} | {}", cmd.1 * cmd.2, count, cmd.0);
          } else {
            line_number += cmd.1 * cmd.2;
            // println!("jmp {} | {}", cmd.1 * cmd.2, count);
          }
        }
        "nop" => {
          if modded && !modded_run {
            modded_run = true;
            line_number += cmd.1 * cmd.2;
            // println!("jmp {} | {} | {}", cmd.1 * cmd.2, count, cmd.0);
          } else {
            line_number += 1;
            // println!("nop {} | {}", cmd.1 * cmd.2, count);
          }
        }
        &_ => {
          error = true;
        }
      }
      if line_number as usize > max - 1 {
        done = true;
      } else if trace.iter().position(|&x| x == line_number).unwrap_or(oob) != oob {
        if !mod_check {
          looped = true;
        } else {
          println!("{} /  {}", super_count, max);
          super_count += 1;
          if super_count as usize >= max  {
            error = true;
          }
          // println!("\nresetting\n");
          modded = false;
          modded_run = false;
          trace = Vec::new();
          line_number = 0;
          acc = 0;
          count = 0;
        }
      }
    }
  }
  println!("state check error {} loop {} done {} super {}", error, looped, done, super_count);
  println!("line {} acc {}", line_number, acc);
 
}

