// You are given an integer array nums.

// Split the array into exactly two subarrays, left and right, such that left is strictly increasing and right is strictly decreasing.

// Return the minimum possible absolute difference between the sums of left and right. If no valid split exists, return -1.

struct Solution;
impl Solution {
  pub fn split_array(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let (mut leftIdx, mut leftSum) = (0usize, nums[0] as i64);
    let (mut rightIdx, mut rightSum) = (0usize, nums[n-1] as i64);

    for idx in 1..n {
      // Increase stops
      if nums[idx] <= nums[idx - 1] {
        leftIdx = idx - 1;
        break;
      }
      leftSum += nums[idx] as i64;
      if idx == n - 1 {
        leftIdx = n - 1;
      }
    }

    for idx in (0..n - 1).rev() {
      // Reverse increase stops
      if nums[idx] <= nums[idx + 1] {
        rightIdx = idx + 1;
        break;
      }
      rightSum += nums[idx] as i64;
      if idx == 0 {
        rightIdx = 0;
      }
    }

    match (leftIdx, rightIdx) {
      // Perfect split only
      (l, r) if r - l == 1  => {
        (leftSum - rightSum).abs()
      },
      // See which side gets middle number
      (l, r) if l == r  => {
        let mid = nums[l] as i64;
        ((leftSum - mid - rightSum).abs()).min(
          (rightSum - mid - leftSum).abs()
        )
      },
      // No split possible
      _  => -1
    }
  }
}