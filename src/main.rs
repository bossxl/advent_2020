#![feature(destructuring_assignment)]
use std::fs;

fn main() {
    println!("Hello, world!");
    day_(false);
}

fn day_(validate:bool) {
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day5.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  let mut ids = vec![];
  let max = contents
    .lines();
 
}

