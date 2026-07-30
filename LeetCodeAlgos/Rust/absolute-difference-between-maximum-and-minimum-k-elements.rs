// You are given an integer array nums and an integer k.

// Find the absolute difference between:

// the sum of the k largest elements in the array; and
// the sum of the k smallest elements in the array.
// Return an integer denoting this difference.

// Constraints:
// 1 <= n == nums.length <= 100
// 1 <= nums[i] <= 100
// 1 <= k <= n

struct Solution;
impl Solution {
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        if k == n {
            return 0;
        }

        nums.sort_unstable();
        for idx in 1..n {
            nums[idx] += nums[idx - 1];
        }
        (nums[n - 1] - nums[n - k - 1]) - nums[k - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 2, 2, 4];
        let k = 2;
        assert_eq!(Solution::abs_difference(nums, k), 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![100];
        let k = 1;
        assert_eq!(Solution::abs_difference(nums, k), 0);
    }
}
