/**
 * Find the two entries that sum to 2020 and
 * then multiply those two numbers together
 */
use std::fs;

fn main() {
  let nums = input_numbers("input.txt".to_string());
  let pair_result = find_pair_result(&nums);
  let tri_result = find_tri_result(&nums);

  println!("Pair result: {}", pair_result);
  println!("Tri result: {}", tri_result);
}

fn input_numbers(filepath: String) -> Vec<i32> {
  let raw: String = fs::read_to_string(filepath).unwrap();
  let strings: Vec<i32> = raw.split("\n").collect::<Vec<&str>>()
    .iter()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  return strings;
}

fn find_pair_result(nums: &Vec<i32>) -> i32 {
  for s in nums {
    for s2 in nums {
      if s + s2 == 2020 {
        return s * s2;
      }
    }
  }

  return -1;
}

fn find_tri_result(nums: &Vec<i32>) -> i32 {
  for s in nums {
    for s2 in nums {
      for s3 in nums {
        if s + s2 + s3 == 2020 {
          return s * s2 * s3;
        }
      }
    }
  }

  return -1;
}
