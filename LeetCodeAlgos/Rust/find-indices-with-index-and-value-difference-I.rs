// You are given a 0-indexed integer array nums having length n, an integer indexDifference, and an integer valueDifference.

// Your task is to find two indices i and j, both in the range [0, n - 1], that satisfy the following conditions:

// abs(i - j) >= indexDifference, and
// abs(nums[i] - nums[j]) >= valueDifference
// Return an integer array answer, where answer = [i, j] if there are two such indices, and answer = [-1, -1] otherwise. If there are multiple choices for the two indices, return any of them.

// Note: i and j may be equal.

// Constraints:
// 1 <= n == nums.length <= 100
// 0 <= nums[i] <= 50
// 0 <= indexDifference <= 100
// 0 <= valueDifference <= 50

struct Solution;
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let n = nums.len();
        if index_difference as usize >= n {
            return vec![-1, -1];
        }
        for i in 0..(n - index_difference as usize) {
            for j in (i + index_difference as usize)..n {
                if (nums[i] - nums[j]).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 1, 4, 1];
        let index_difference = 2;
        let value_difference = 4;
        let result = Solution::find_indices(nums, index_difference, value_difference);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 1];
        let index_difference = 0;
        let value_difference = 0;
        let result = Solution::find_indices(nums, index_difference, value_difference);
        assert_eq!(result, vec![0, 0]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        let index_difference = 2;
        let value_difference = 4;
        let result = Solution::find_indices(nums, index_difference, value_difference);
        assert_eq!(result, vec![-1, -1]);
    }
}
