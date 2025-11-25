// You are given a 0-indexed integer array nums and two integers key and k. A k-distant index is an index i of nums for which there exists at least one index j such that |i - j| <= k and nums[j] == key.

// Return a list of all k-distant indices sorted in increasing order.

struct Solution;

impl Solution {
  pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::<i32>::new();

    for j in 0..n {
      if nums[j] == key {
        // If key found, get all indices within range
        // that are not already gotten
        let last = ans.last().unwrap_or(&-1);
        for i in (j as i32 - k).max(last + 1)..=(j as i32 + k) {
          if i >= 0 && i < n as i32 {
            ans.push(i);
          }
        }
      }
    }

    ans
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
      vec![1, 2, 3, 4, 5, 6]
    );
  }

  #[test]
  fn example2() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
      vec![0, 1, 2, 3, 4]
    );
  }

  #[test]
  fn empty_array() {
    assert_eq!(
      Solution::find_k_distant_indices(Vec::<i32>::new(), 1, 1),
      Vec::<i32>::new()
    );
  }

  #[test]
  fn key_at_ends() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![5, 1, 2, 3, 5], 5, 1),
      vec![0, 1, 3, 4]
    );
  }

  #[test]
  fn large_k_covers_all_indices() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![1, 2, 3], 2, 10),
      vec![0, 1, 2]
    );
  }

  #[test]
  fn repeated_noncontiguous_keys() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![1, 9, 1, 9, 1], 9, 1),
      vec![0, 1, 2, 3, 4]
    );
  }

  #[test]
  fn single_element_key() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![7], 7, 0),
      vec![0]
    );
  }

  #[test]
  fn negative_numbers() {
    assert_eq!(
      Solution::find_k_distant_indices(vec![-1, -2, -1, -3], -1, 1),
      vec![0, 1, 2, 3]
    );
  }
}
