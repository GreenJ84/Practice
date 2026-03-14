// You are given an integer array nums.

// The alternating sum of nums is the value obtained by adding elements at even indices and subtracting elements at odd indices. That is, nums[0] - nums[1] + nums[2] - nums[3]...

// Return an integer denoting the alternating sum of nums.

struct Solution;
impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
      nums.iter()
        .enumerate()
        .fold(0, |count, (idx, val)| {
          if idx % 2 == 0 {
            count + val
          } else {
            count - val
          }
        })
    }

    pub fn _alternating_sum1(nums: Vec<i32>) -> i32 {
      nums.iter()
        .enumerate()
        .map(|(idx, val)| {
          if idx % 2 == 1 {
            return -*val;
          }
          *val
        })
        .sum()
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![1,3,5,7];
    let result = Solution::alternating_sum(nums);
    assert_eq!(result, -4);
  }

  #[test]
  fn test2() {
    let nums = vec![100];
    let result = Solution::alternating_sum(nums);
    assert_eq!(result, 100);
  }
}