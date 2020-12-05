use std::fs;

fn main() {
    println!("Hello, world!");
    println!("Sen 1");
    day_(false);
    // println!("Sen 2");
    // day_four(true);
}

fn day_(validate:bool) -> i32{
  let mut total_valid = 0;
  let mut total = 0;
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day4.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
  let lines: Vec<&str> = contents
    .lines()
    .collect();
  
}

