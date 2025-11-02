// You are given an array of integers nums. Consider the following operation:

// Delete the first two elements nums and define the score of the operation as the sum of these two elements.
// You can perform this operation until nums contains fewer than two elements. Additionally, the same score must be achieved in all operations.

// Return the maximum number of operations you can perform.

struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
      let n = nums.len();
      let target = nums[0] + nums[1];
      let mut ans = 1;

      for idx in (3..n).step_by(2) {
        if nums[idx] + nums[idx - 1] != target {
            return ans;
        }
        ans += 1;
      }
      ans
    }
}