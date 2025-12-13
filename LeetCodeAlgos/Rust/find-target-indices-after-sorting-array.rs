// You are given a 0-indexed integer array nums and a target element target.

// A target index is an index i such that nums[i] == target.

// Return a list of the target indices of nums after sorting nums in non-decreasing order. If there are no target indices, return an empty list. The returned list must be sorted in increasing order.

struct Solution;
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
      let mut less_than = 0i32;
      let mut equal = 0i32;
      nums.iter().for_each(|n| {
        if n < &target { less_than += 1; }
        else if n == &target { equal += 1; }
      });
      (less_than..(less_than + equal)).collect()
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let nums = vec![1, 2, 5, 2, 3];
    let target = 2;
    assert_eq!(Solution::target_indices(nums, target), vec![1, 2]);
  }

  #[test]
  fn example_2() {
    let nums = vec![1, 2, 5, 2, 3];
    let target = 3;
    assert_eq!(Solution::target_indices(nums, target), vec![3]);
  }

  #[test]
  fn example_3() {
    let nums = vec![1, 2, 5, 2, 3];
    let target = 5;
    assert_eq!(Solution::target_indices(nums, target), vec![4]);
  }

  #[test]
  fn single_element_match() {
    let nums = vec![5];
    let target = 5;
    assert_eq!(Solution::target_indices(nums, target), vec![0]);
  }

  #[test]
  fn single_element_no_match() {
    let nums = vec![5];
    let target = 3;
    assert_eq!(Solution::target_indices(nums, target), vec![]);
  }

  #[test]
  fn all_elements_same() {
    let nums = vec![2, 2, 2, 2];
    let target = 2;
    assert_eq!(Solution::target_indices(nums, target), vec![0, 1, 2, 3]);
  }

  #[test]
  fn no_match() {
    let nums = vec![1, 3, 4, 5];
    let target = 2;
    assert_eq!(Solution::target_indices(nums, target), vec![]);
  }

  #[test]
  fn target_at_start() {
    let nums = vec![5, 3, 1, 2];
    let target = 1;
    assert_eq!(Solution::target_indices(nums, target), vec![0]);
  }

  #[test]
  fn multiple_targets() {
    let nums = vec![1, 1, 1, 1, 1];
    let target = 1;
    assert_eq!(Solution::target_indices(nums, target), vec![0, 1, 2, 3, 4]);
  }
}