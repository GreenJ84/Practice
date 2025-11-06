// Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.

// The value of |x| is defined as:

// x if x >= 0.
// -x if x < 0.

struct Solution;
impl Solution {
  pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;

    for left in 0..n-1 {
      for right in left+1..n {
        if (nums[left] - nums[right]).abs() == k {
          ans += 1;
        }
      }
    }
    ans
  }
}