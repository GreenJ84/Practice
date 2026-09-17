// Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).

// Specifically, ans is the concatenation of two nums arrays.

// Return the array ans.

// Constraints:
// n == nums.length
// 1 <= n <= 1000
// 1 <= nums[i] <= 1000

struct Solution;
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 1];
        let expected = vec![1, 2, 1, 1, 2, 1];
        assert_eq!(Solution::get_concatenation(nums), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 2, 1];
        let expected = vec![1, 3, 2, 1, 1, 3, 2, 1];
        assert_eq!(Solution::get_concatenation(nums), expected);
    }
}
 