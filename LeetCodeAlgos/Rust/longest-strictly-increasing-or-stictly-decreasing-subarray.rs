// You are given an array of integers nums. Return the length of the longest subarray of nums which is either strictly increasing or strictly decreasing.

struct Solution;

use std::cmp::Ordering;
impl Solution {
  pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 1;

        let mut window_start = 0;
        let mut pattern = Ordering::Equal;
        for idx in 1..nums.len() {
          let diff = nums[idx].cmp(&nums[idx - 1]);

          match (diff, pattern) {
            (Ordering::Equal, Ordering::Equal) => {
              window_start = idx;
            },
            (Ordering::Equal, _) | (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less) => {
              ans = ans.max(idx - window_start);
              window_start = idx - 1;
              if diff == Ordering::Equal {
                window_start += 1;
              }
              pattern = diff;
            },
            (_, Ordering::Equal) => {
              pattern = diff;
            },
            _ => {}
          }
        }
        ans.max(nums.len() - window_start) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1,4,3,3,2];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![3,3,3,3];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![3,2,1];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 3);
    }

        #[test]
    fn test_4() {
        let nums = vec![1,9,7,1];
        assert_eq!(Solution::longest_monotonic_subarray(nums), 3);
    }
}
