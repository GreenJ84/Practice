// You are given an integer array nums.

// You replace each element in nums with the sum of its digits.

// Return the minimum element in nums after all replacements.

struct Solution;
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
    nums.iter()
      .map(|&num| {
        Self::digit_sum(num)
      })
      .min()
      .unwrap()
  }

  fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
      sum += n % 10;
      n /= 10;
    }
    sum
  }
}