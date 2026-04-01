// You are given an integer array nums of length n.

// An array is trionic if there exist indices 0 < p < q < n − 1 such that:

// nums[0...p] is strictly increasing,
// nums[p...q] is strictly decreasing,
// nums[q...n − 1] is strictly increasing.
// Return true if nums is trionic, otherwise return false.

struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums[0] >= nums[1] || nums[nums.len() - 2] >= nums[nums.len() - 1] {
            return false;
        }
        let mut flips = 0;

        for i in 2..nums.len() {
          match (flips % 2, nums[i - 1] < nums[i]){
            (0, true) => {},
            (0, false) => {
              if nums[i - 1] == nums[i] || flips >= 2 {
                return false;
              }
              flips += 1;
            },
            (_, false) => {
              if nums[i - 1] == nums[i] {
                return false;
              }
            }
            (_, true) => {
              flips += 1;
            }
          }
        }
        return flips == 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1,3,5,4,2,6];
        assert_eq!(Solution::is_trionic(nums), true);
    }

    #[test]
    fn test_2() {
        let nums = vec![2,1,3];
        assert_eq!(Solution::is_trionic(nums), false);
    }
}

// Example 1:

// Input: nums = [1,3,5,4,2,6]

// Output: true

// Explanation:

// Pick p = 2, q = 4:

// nums[0...2] = [1, 3, 5] is strictly increasing (1 < 3 < 5).
// nums[2...4] = [5, 4, 2] is strictly decreasing (5 > 4 > 2).
// nums[4...5] = [2, 6] is strictly increasing (2 < 6).
// Example 2:

// Input: nums = [2,1,3]

// Output: false

// Explanation:

// There is no way to pick p and q to form the required three segments.