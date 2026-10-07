// You are given a 0-indexed integer array nums of even length.

// As long as nums is not empty, you must repetitively:

// Find the minimum number in nums and remove it.
// Find the maximum number in nums and remove it.
// Calculate the average of the two removed numbers.
// The average of two numbers a and b is (a + b) / 2.

// For example, the average of 2 and 3 is (2 + 3) / 2 = 2.5.
// Return the number of distinct averages calculated using the above process.

// Note that when there is a tie for a minimum or maximum number, any can be removed.

// Constraints:
// 2 <= nums.length <= 100
// nums.length is even.
// 0 <= nums[i] <= 100

struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        (0..n / 2)
            .map(|i| nums[i] + nums[n - i - 1])
            .collect::<HashSet<_>>()
            .len() as i32
    }

    pub fn distinct_averages1(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        HashSet::<i32>::from_iter((0..n / 2).map(|i| nums[i] + nums[n - i - 1])).len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 1, 4, 0, 3, 5];
        let result = Solution::distinct_averages(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 100];
        let result = Solution::distinct_averages(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7];
        let result = Solution::distinct_averages(nums);
        assert_eq!(result, 5);
    }
}
