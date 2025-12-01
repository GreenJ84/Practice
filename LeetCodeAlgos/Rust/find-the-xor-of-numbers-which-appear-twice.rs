// You are given an array nums, where each number in the array appears either once or twice.

// Return the bitwise XOR of all the numbers that appear twice in the array, or 0 if no number appears twice.

struct Solution;

use std::collections::HashMap;
impl Solution {
  pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    let mut xor = 0;

    nums.iter().for_each(|&num| {
      map.entry(num)
        .and_modify(|_| {
          xor ^= num;
        })
        .or_insert(1);
    });
    xor
  }

  pub fn duplicate_numbers_xor1(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    let mut xor = 0;

    for num in nums {
      map.entry(num)
        .and_modify(|_| {
          xor ^= num;
        })
        .or_insert(1);
    }
    xor
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example_1() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
  }

  #[test]
  fn example_3() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
  }

  #[test]
  fn single_element_no_duplicate() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![5]), 0);
  }

  #[test]
  fn two_same_elements() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![7, 7]), 7);
  }

  #[test]
  fn mixed_duplicates() {
    // duplicates are 4 and 6 -> 4 ^ 6 = 2
    assert_eq!(Solution::duplicate_numbers_xor(vec![4, 5, 6, 4, 7, 6, 8]), 2);
  }

  #[test]
  fn one_duplicate_with_others_unique() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![50, 50, 1, 2, 3]), 50);
  }

  #[test]
  fn all_unique_max_length() {
    // nums = [1, 2, ..., 50], all appear once -> result 0
    let nums: Vec<i32> = (1..=50).collect();
    assert_eq!(Solution::duplicate_numbers_xor(nums), 0);
  }
}