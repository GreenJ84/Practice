// Given an array of positive integers nums, return the maximum possible sum of an strictly increasing subarray in nums.

// A subarray is defined as a contiguous sequence of numbers in an array.

struct Solution;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
      let mut max = 0;
      let mut current = nums[0];
      for idx in 1..nums.len() {
        if nums[idx] <= nums[idx - 1] {
          max = max.max(current);
          current = 0;
        }
        current += nums[idx];
      }
      max.max(current)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
  }

  #[test]
  fn example_3() {
    assert_eq!(Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
  }

  #[test]
  fn single_element() {
    assert_eq!(Solution::max_ascending_sum(vec![42]), 42);
  }

  #[test]
  fn all_descending() {
    assert_eq!(Solution::max_ascending_sum(vec![50, 40, 30, 20, 10]), 50);
  }

  #[test]
  fn all_same() {
    assert_eq!(Solution::max_ascending_sum(vec![5, 5, 5, 5]), 5);
  }

  #[test]
  fn two_ascending() {
    assert_eq!(Solution::max_ascending_sum(vec![1, 2]), 3);
  }

  #[test]
  fn multiple_ascending_sequences() {
    assert_eq!(Solution::max_ascending_sum(vec![1, 2, 3, 2, 3, 4, 5]), 14);
  }

  #[test]
  fn boundary_values() {
    assert_eq!(Solution::max_ascending_sum(vec![1, 100]), 101);
  }
}