fn day_two(){
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day2.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  let mut total = 0;
  let passwords: Vec<&str> = contents
      .lines()
      .filter_map(|s| 
        if check_value (s) {
          total += 1;
          Some(s)
        } else {
          total += 1;
          None
        }
    ).collect();
  println!("total {}", total);
  println!("count {}", passwords.len());
}

fn check_value(data:&str) -> bool {
  let password_parts: Vec<&str> = data.split_whitespace().collect();
  let counts: Vec<&str> = password_parts[0].split("-").collect();
  let low:usize = String::from(counts[0]).parse::<usize>().unwrap() - 1;
  let high:usize = String::from(counts[1]).parse::<usize>().unwrap() - 1;
  let character: Vec<char> = String::from(password_parts[1]).replace(":","").chars().collect();
  let password = String::from(password_parts[2]);
  // let mut total = 0;
  let p_chars: Vec<char> = password.chars().collect();
  let the_char = character[0];
  let mut passed = false;
  if p_chars[low] == the_char {
    passed = !passed;
  }
  if p_chars[high] == the_char {
    passed = !passed;
  }
  return passed;
  // for c in password.chars() { 
  //   // do something with `c`
  //   if c == character[0] {
  //     total += 1;
  //   }
  // }
  // if(total >= low && total <= high){
  //   return true;
  // }
  // return false;
}