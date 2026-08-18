// Given a 0-indexed integer array nums of size n, find the maximum difference between nums[i] and nums[j] (i.e., nums[j] - nums[i]), such that 0 <= i < j < n and nums[i] < nums[j].

// Return the maximum difference. If no such i and j exists, return -1.

// Constraints:
// n == nums.length
// 2 <= n <= 1000
// 1 <= nums[i] <= 109

struct Solution;
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut ans = -1;
        for i in 1..nums.len() {
            if min < nums[i] {
                ans = ans.max(nums[i] - min);
            }
            min = min.min(nums[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![7, 1, 5, 4];
        assert_eq!(Solution::maximum_difference(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![9, 4, 3, 2];
        assert_eq!(Solution::maximum_difference(nums), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 5, 2, 10];
        assert_eq!(Solution::maximum_difference(nums), 9);
    }
}
