#![feature(destructuring_assignment)]
use std::fs;

fn main() {
    println!("Hello, world!");
    println!("Sen 1");
    day_5(false);
    // println!("Sen 2");
    // day_four(true);
}

fn process_letter(letter: &str, low: i32, high: i32) -> (i32, i32) {
  let mut new_low = low;
  let mut new_high = high;
  // let mid: i32 = ((high + low) as f32 / 2.0).ceil() as i32;
  let mid: i32 = (high + low) / 2;
  
  println!("looking at [{}] l: {} h: {} m: {}", letter, low, high, mid);
  match letter {
    "F" => { new_high = mid }
    "B" => { new_low = mid }
    "L" => { new_high = mid }
    "R" => { new_low = mid }
    _ => { }
  }
  

  return (new_low, new_high);
}

fn process_seat(seat_id: &str) -> i32 {
  let mut high_row = 127;
  let mut low_row = 0;
  let mut high_col = 7;
  let mut low_col = 0;
  for letter in seat_id.split("") {
    match letter {
      "F" | "B" => { 
        (low_row, high_row ) = process_letter(letter, low_row, high_row);
        println!("result row at [{}] l: {} h: {}\n", letter, low_row, high_row);
      }
      "L" | "R" => { 
        (low_col, high_col ) = process_letter(letter, low_col, high_col);
        println!("result col at [{}] l: {} h: {}\n", letter, low_col, high_col);
      }
      _ => { }
    }
    
  }
  let result = high_row * 8 + high_col;
  println!("result {}\n\n", result);
  return result;
}

fn day_5(validate:bool) {
  let mut total_valid = 0;
  let mut total = 0;
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day5.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
  let max = contents
    .lines()
    .map(|f| process_seat(f) )
    .max()
    .unwrap_or(0);
  println!("max {}", max);
  
}

