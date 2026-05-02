// You are given an integer array nums consisting only of 0, 1, and 2.

// A pair of indices (i, j) is called valid if nums[i] == 1 and nums[j] == 2.

// Return the minimum absolute difference between i and j among all valid pairs. If no valid pair exists, return -1.

// The absolute difference between indices i and j is defined as abs(i - j).

// Constraints:
// - 1 <= nums.length <= 100
// - 0 <= nums[i] <= 2

struct Solution;
impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut min_diff = i32::MAX;
        let mut last: (i32, usize) = (nums[0], 0);

        for idx in 0..nums.len() {
            match nums[idx] {
                0 => {}
                n if last.0 == 0 => { last = (n, idx); },
                n if last.0 == n => {
                  last.1 = idx;
                },
                n => {
                  min_diff = min_diff.min((idx - last.1) as i32);
                  last = (n, idx);
                }
            }
        }

        if min_diff == i32::MAX {
          -1
        } else {
          min_diff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 0, 0, 2, 0, 1];
        assert_eq!(Solution::min_absolute_difference(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 0, 1, 0];
        assert_eq!(Solution::min_absolute_difference(nums), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 2, 1, 0];
        assert_eq!(Solution::min_absolute_difference(nums), 1);
    }
}
