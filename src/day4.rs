use std::fs;

fn main() {
    println!("Hello, world!");
    println!("Sen 1");
    day_four(false);
    println!("Sen 2");
    day_four(true);
}

fn validate_byr(value: &str) -> bool {
  let year = value.parse::<i32>().unwrap();
  if year >= 1920 && year <= 2002 {
    return true
  }
  return false;
}
fn validate_iyr(value: &str) -> bool {
  let year = value.parse::<i32>().unwrap();
  if year >= 2010 && year <= 2020 {
    return true
  }
  return false;
}
fn validate_eyr(value: &str) -> bool {
  let year = value.parse::<i32>().unwrap();
  if year >= 2020 && year <= 2030 {
    return true
  }
  return false;
}
fn validate_hgt(value: &str) -> bool {
  if value.contains("cm") {
    let cleaned = value.replace("cm", "");
    let height: i32 = cleaned.parse().unwrap();
    if height >= 150 && height <= 193 {
      return true
    }
  } else if value.contains("in") {
    let cleaned = value.replace("in", "");
    let height: i32 = cleaned.parse().unwrap();
    if height >= 59 && height <= 76 {
      return true
    }
  }
  return false;
}
fn validate_hcl(value: &str) -> bool {
  if !value.contains("#") { 
    return false
  }
  let mut _valid = false;
  let without_prefix = value.trim_start_matches("#");
  match  i64::from_str_radix(without_prefix, 16) {
    Ok(n) => { _valid = true; }
    Err(_) => { _valid = false; }
  }
  return _valid;
}
fn validate_ecl(value: &str) -> bool {
  let mut _valid = false;
  match value {
    "amb" => { _valid = true } 
    "blu"  => { _valid = true } 
    "brn"  => { _valid = true } 
    "gry"  => { _valid = true } 
    "grn"  => { _valid = true } 
    "hzl"  => { _valid = true } 
    "oth" => { _valid = true } 
    _ => { _valid = false}
  }
  return _valid;
}
fn validate_pid(value: &str) -> bool {
  if value.len() != 9 {
    return false
  }
  for c in value.chars() {
    if !c.is_numeric() {
        return false;
    }
  }
  return true;
}
fn process_passport(passport: &str, validate: bool) -> bool {
  let valid_attributes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
  let mut required_attributes = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  let mut valid = false;
  let mut bad_data = false;
  println!("processing passport {}", passport);
  let passport_parts: Vec<&str>  = passport.split_whitespace().collect();
  for part in passport_parts {
    let mut p_data = part.split(":");
    let field: &str = p_data.next().unwrap_or("XXINVALIDXX");
    if valid_attributes.iter().any(|&i| i == field) {
      let index = required_attributes.iter().position(|x| *x == field).unwrap_or(404);
      if index != 404 {
        required_attributes.remove(index);
      }
      let value: &str = p_data.next().unwrap_or("XXINVALIDXX");
      if validate {
        match field {
          "byr" => { 
            if !validate_byr(value) {
              bad_data = true;
            }
          }
          "iyr"=> { 
            if !validate_iyr(value) {
              bad_data = true;
            }
          }
          "eyr"=> { 
            if !validate_eyr(value) {
              bad_data = true;
            }
          }
          "hgt"=> { 
            if !validate_hgt(value) {
              bad_data = true;
            }
          }
          "hcl"=> { 
            if !validate_hcl(value) {
              bad_data = true;
            }
          }
          "ecl"=> { 
            if !validate_ecl(value) {
              bad_data = true;
            }
          }
          "pid"=> { 
            if !validate_pid(value) {
              bad_data = true;
            }
          }
          _ => { }
        }
        if bad_data {
          break;
        }
      }
    } else {
      break;
    }
  }
  if !bad_data && required_attributes.len() == 0 {
    valid = true;
  } else {
    println!("missing attribute / bad data");
  }
  if valid {
    // println!("✔ valid passport {}\n", passport)
    println!("✔ valid passport \n")
  } else {
    // println!("✗ invalid passport {}\n", passport)
    println!("✗ invalid passport \n ")
  }
  return valid;

}

fn day_four(validate:bool) -> i32{
  let mut total_valid = 0;
  let mut total = 0;
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day4.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
  let lines: Vec<&str> = contents
    .lines()
    .collect();
  let mut passport = "".to_owned();
  for (i, line) in lines.iter().enumerate() {
    println!("buildig passport {}", passport);
    if passport.len() > 0 {
      passport.push_str(" ");
    }
    passport.push_str(line);
    // println!("line {} / {}", i, lines.len());
    if line.len() == 0 || i == lines.len() - 1 {
      
      total += 1;
      if passport.len() > 0 && process_passport(&passport, validate) == true {
        total_valid += 1;
      }
      passport = "".to_owned();
      continue;
    }

  }

  println!("total valid passports {}", total_valid);
  println!("total passports {}", total);
  return total_valid;
}

