// You are given an integer array nums of size n. For each index i where 0 <= i < n, define a subarray nums[start ... i] where start = max(0, i - nums[i]).

// Return the total sum of all elements from the subarray defined for each index in the array.

struct Solution;
impl Solution {
  pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut pre_sum = Vec::<i32>::with_capacity(nums.len());
    pre_sum.push(nums[0]);
    for idx in 1..nums.len() {
        pre_sum.push(pre_sum[idx - 1] + nums[idx]);
    }

    let mut sum = 0;
    for idx in 0..nums.len() {
      sum += pre_sum[idx];
      let start =  idx.checked_sub(nums[idx] as usize);
      if start > Some(0) {
        sum -= pre_sum[start.unwrap() - 1];
      }
    }
    sum
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(Solution::subarray_sum(vec![2, 3, 1]), 11);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::subarray_sum(vec![3, 1, 1, 2]), 13);
  }

  #[test]
  fn single_element() {
    assert_eq!(Solution::subarray_sum(vec![5]), 5);
  }

  #[test]
  fn all_ones() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1]), 5);
  }

  #[test]
  fn large_first_element() {
    assert_eq!(Solution::subarray_sum(vec![1000, 1, 1]), 2003);
  }

  #[test]
  fn increasing_array() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4]), 20);
  }

  #[test]
  fn decreasing_array() {
    assert_eq!(Solution::subarray_sum(vec![4, 3, 2, 1]), 23);
  }
}