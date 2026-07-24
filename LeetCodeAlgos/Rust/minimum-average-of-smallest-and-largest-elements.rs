// You have an array of floating point numbers averages which is initially empty. You are given an array nums of n integers where n is even.

// You repeat the following procedure n / 2 times:

// Remove the smallest element, minElement, and the largest element maxElement, from nums.
// Add (minElement + maxElement) / 2 to averages.
// Return the minimum element in averages.

// Constraints:
// 2 <= n == nums.length <= 50
// n is even.
// 1 <= nums[i] <= 50

struct Solution;
impl Solution {
    pub fn minimum_average1 (mut nums: Vec<i32>) -> f64 {
      let n: usize = nums.len();
      nums.sort_unstable();

      let mut ans: f64 = f64::MAX;
      for i in 0..(n / 2) {
        let avg: f64 = (nums[i] + nums[n - i - 1]) as f64 / 2.0;
        if avg < ans { ans = avg; }
      }
      ans
    }

    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
      let n: usize = nums.len();
      let mut ans: f64 = f64::MAX;
      nums.sort_unstable();

      for i in 0..(n / 2) {
        ans = ans.min( (nums[i] + nums[n - i - 1]) as f64 / 2.0 );
      }
      ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let nums = vec![7,8,3,4,15,13,4,1];
    assert_eq!(Solution::minimum_average(nums), 5.5);
  }

  #[test]
  fn test_2() {
    let nums = vec![1,9,8,3,10,5];
    assert_eq!(Solution::minimum_average(nums), 5.5);
  }
  
  #[test]
  fn test_3() {
    let nums = vec![1,2,3,7,8,9];
    assert_eq!(Solution::minimum_average(nums), 5.0);
  }
}
