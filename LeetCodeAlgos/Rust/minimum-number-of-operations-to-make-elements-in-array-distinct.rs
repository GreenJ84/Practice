// You are given an integer array nums. You need to ensure that the elements in the array are distinct. To achieve this, you can perform the following operation any number of times:

// Remove 3 elements from the beginning of the array. If the array has fewer than 3 elements, remove all remaining elements.
// Note that an empty array is considered to have distinct elements. Return the minimum number of operations needed to make the elements in the array distinct.

struct Solution;
impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut seen = std::collections::HashSet::<&i32>::new();
    for idx in (0..nums.len()).rev() {
      if !seen.insert(&nums[idx]) {
        return (idx / 3 + 1) as i32;
      }
    }
    0
  }

  pub fn minimum_operations1(nums: Vec<i32>) -> i32 {
    let mut seen = std::collections::HashSet::<&i32>::new();
    for idx in (0..nums.len()).rev() {
      if seen.contains(&&nums[idx]) {
        if (idx + 1) % 3 == 0 {
          return ((idx + 1) / 3) as i32;
        }
        return ((idx + 1) / 3 + 1) as i32;
      }
      seen.insert(&nums[idx]);
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::minimum_operations(vec![1,2,3,4,2,3,3,5,7]), 2);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::minimum_operations(vec![4,5,6,4,4]), 2);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::minimum_operations(vec![6,7,8,9]), 0);
  }

  #[test]
  fn test_single_element() {
    assert_eq!(Solution::minimum_operations(vec![1]), 0);
  }

  #[test]
  fn test_all_duplicates() {
    assert_eq!(Solution::minimum_operations(vec![1,1,1,1]), 1);
  }

  #[test]
  fn test_two_elements_distinct() {
    assert_eq!(Solution::minimum_operations(vec![1,2]), 0);
  }

  #[test]
  fn test_two_elements_duplicate() {
    assert_eq!(Solution::minimum_operations(vec![1,1]), 1);
  }

  #[test]
  fn test_large_array_needs_multiple_ops() {
    assert_eq!(Solution::minimum_operations(vec![1,2,3,1,2,3,1,2,3]), 2);
  }
}
