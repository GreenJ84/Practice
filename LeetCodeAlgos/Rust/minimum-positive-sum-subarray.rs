// You are given an integer array nums and two integers l and r. Your task is to find the minimum sum of a subarray whose size is between l and r (inclusive) and whose sum is greater than 0.

// Return the minimum sum of such a subarray. If no such subarray exists, return -1.

// A subarray is a contiguous non-empty sequence of elements within an array.

struct Solution {}
impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
      // pre-sum makes for more efficient sum lookups
      let n = nums.len();
      let mut pre_sum = vec![0; n + 1];
      for i in 0..n {
        pre_sum[i + 1] = pre_sum[i] + nums[i];
      }
      let (l, r) = (l as usize, r as usize);
      let mut min_sum: Option<i32> = None;

    // traverse possible window sizes
    for window_size in l..=r {
      // traverse possible starting indices
      for start in 0..=n - window_size {
        let end = start + window_size;
        // if end is over array len, go to next window size
        if end > n {
          break;
        }
        let sum = pre_sum[end] - pre_sum[start];
        if sum > 0 {
          match min_sum {
            Some(current_min) => {
                if sum < current_min {
                    min_sum = Some(sum);
                }
            }
            None => {
                min_sum = Some(sum);
            }
          }
        }
      }
    }
    min_sum.unwrap_or(-1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_minimum_sum_subarray() {
    assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
    assert_eq!(Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
    assert_eq!(Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
  }
}