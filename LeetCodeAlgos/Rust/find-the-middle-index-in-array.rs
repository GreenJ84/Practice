// Given a 0-indexed integer array nums, find the leftmost middleIndex (i.e., the smallest amongst all the possible ones).

// A middleIndex is an index where nums[0] + nums[1] + ... + nums[middleIndex-1] == nums[middleIndex+1] + nums[middleIndex+2] + ... + nums[nums.length-1].

// If middleIndex == 0, the left side sum is considered to be 0. Similarly, if middleIndex == nums.length - 1, the right side sum is considered to be 0.

// Return the leftmost middleIndex that satisfies the condition, or -1 if there is no such index.

struct Solution;
impl Solution {
  pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut right = nums.iter().sum::<i32>() - nums[0];
    if right == 0 { return 0; }

    let mut left = 0;
    for idx in 1..nums.len(){
      right -= nums[idx];
      left += nums[idx - 1];
      if left == right { return idx as i32; }
    }
    -1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
  }

  #[test]
  fn test_single_element() {
    assert_eq!(Solution::find_middle_index(vec![1]), 0);
  }

  #[test]
  fn test_middle_at_start() {
    assert_eq!(Solution::find_middle_index(vec![0, 1, 2, -3]), 0);
  }

  #[test]
  fn test_middle_at_end() {
    assert_eq!(Solution::find_middle_index(vec![-3, 2, 1, 0]), 3);
  }

  #[test]
  fn test_all_zeros() {
    assert_eq!(Solution::find_middle_index(vec![0, 0, 0, 0]), 0);
  }

  #[test]
  fn test_negative_numbers() {
    assert_eq!(Solution::find_middle_index(vec![-1, -1, -1, -1, -1, 0]), 2);
  }

  #[test]
  fn test_no_middle_index() {
    assert_eq!(Solution::find_middle_index(vec![1, 2, 3, 4]), -1);
  }

  #[test]
  fn test_two_elements_equal() {
    assert_eq!(Solution::find_middle_index(vec![1, 1]), -1);
  }

  #[test]
  fn test_large_numbers() {
    assert_eq!(Solution::find_middle_index(vec![1000, -9000, 500, 500]), 1);
  }

  #[test]
  fn test_multiple_valid_returns_leftmost() {
    assert_eq!(Solution::find_middle_index(vec![0, 0, 0]), 0);
  }
}
