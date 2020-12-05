use std::fs;

fn main() {
    println!("Hello, world!");
    println!("Sen 1");
    day_three(3, 1);
    println!("\nSen 2");
    let slopes = [
      [1,1],
      [3,1],
      [5,1],
      [7,1],
      [1,2]
    ];
    let tree_total: i32 = slopes.iter().map(|slope| day_three(slope[0], slope[1])).product();
    println!("All my trees {}", tree_total);
}

fn day_three(slope_x: usize, slope_y: usize) -> i32{
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day3.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  let mut total_tree = 0;
  let mut x = 0;
  let mut y = 0;

  let hash: &str = "#";
  let tree = hash.chars().next().expect("string is empty");
  let rows: Vec<&str> = contents
      .lines().collect();
  for (count, row) in rows.iter().enumerate() {
    if count != y {
      continue;
    }
    let data: Vec<char> = row.chars().collect();
    let pos = row.chars().nth(x % data.len()).unwrap();
    // println!("Checking for a tree [{}] x {} y {}", pos, x, y);
    if pos == tree {
      total_tree += 1;
    }
    x += slope_x;
    y += slope_y;
  }
  println!("All my slope {},{} trees {}", slope_x, slope_y, total_tree); // 62 // 56
  return total_tree;
}

