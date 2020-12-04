fn day_one() {
  let filename = "/Users/pboss/workspace/advent/Y2020/data/day1.txt";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
  // println!("With text:\n{}", contents);
  let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
  let mut end = false;
  let expected = 2020;
  let mut num1 = 0;
  let mut num2 = 0;
  let mut num3 = 0;
  for (count, number) in numbers.iter().enumerate() {
    num1 = *number;
    for (i_count, i_number) in numbers.iter().enumerate() {
      if(i_count == count){
        continue;
      }
      num2 = *i_number;
      for (ii_count, ii_number) in numbers.iter().enumerate() {
        if(ii_count == count || ii_count == i_count){
          continue;
        }
        num3 = *ii_number;
        let total = num1 + num2 + num3;
        // println!("N1 {} + N2 {} + N3 {} = T{}", num1, num2, num3, total);
        if(total == expected){
          end = true;
          println!("N1 {} + N2 {} + N3 {} = T{}", num1, num2, num3, total);
          println!("value :\n{}", num1 * num2 * num3);
          break;
        }
      }
      if(end){
        break;
      }
    }
    if(end){
      break;
    }
  }
}