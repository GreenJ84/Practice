// Given a 0-indexed integer array nums, return true if it can be made strictly increasing after removing exactly one element, or false otherwise. If the array is already strictly increasing, return true.

// The array nums is strictly increasing if nums[i - 1] < nums[i] for each index (1 <= i < nums.length).

struct Solution;
impl Solution {
  pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    let mut changes = 0;

    for idx in 1..nums.len(){
      // Strictly increasing broken
      if nums[idx] <= nums[idx - 1] {
        // can we remove previous (1 op)
        if idx == 1 || nums[idx] > nums[idx - 2] {
          changes += 1;
        }
        // can we remove current (1 op)
        else if idx + 1 == nums.len() || nums[idx + 1] > nums[idx - 1] {
          changes += 1;
        } else {
          return false;
        }
        // failed increasing test
        if changes > 1 {
          return false;
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
  }

  #[test]
  fn test_example2() {
    assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
  }

  #[test]
  fn test_example3() {
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
  }

  #[test]
  fn test_already_strictly_increasing() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 3, 4, 5]), true);
  }

  #[test]
  fn test_two_elements_increasing() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2]), true);
  }

  #[test]
  fn test_two_elements_decreasing() {
    assert_eq!(Solution::can_be_increasing(vec![2, 1]), true);
  }

  #[test]
  fn test_duplicate_at_end() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 3, 3]), true);
  }

  #[test]
  fn test_duplicate_at_start() {
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 2, 3]), true);
  }

  #[test]
  fn test_all_same_elements() {
    assert_eq!(Solution::can_be_increasing(vec![5, 5, 5, 5]), false);
  }

  #[test]
  fn test_one_large_element() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 100, 3, 4]), true);
  }
}

