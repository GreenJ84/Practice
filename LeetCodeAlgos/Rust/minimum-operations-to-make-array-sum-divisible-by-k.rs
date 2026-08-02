// You are given an integer array nums and an integer k. You can perform the following operation any number of times:

// Select an index i and replace nums[i] with nums[i] - 1.
// Return the minimum number of operations required to make the sum of the array divisible by k.

// Constraints:
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 1000
// 1 <= k <= 100

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 9, 7];
        let k = 5;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 3];
        let k = 4;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2];
        let k = 6;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, 5);
    }
}
