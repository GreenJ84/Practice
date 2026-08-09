// You are given a 0-indexed integer array nums and an integer k. Your task is to perform the following operation exactly k times in order to maximize your score:

// Select an element m from nums.
// Remove the selected element m from the array.
// Add a new element with a value of m + 1 to the array.
// Increase your score by m.
// Return the maximum score you can achieve after performing the operation exactly k times.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
// 1 <= k <= 100

struct Solution;
impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        *nums.iter().max().unwrap() * k + (k - 1) * k / 2
    }

    pub fn maximize_sum1(nums: Vec<i32>, k: i32) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let mut ans = mx;
        for i in 1..k {
            ans += mx + i;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 3;
        assert_eq!(Solution::maximize_sum(nums, k), 18);
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 5, 5];
        let k = 2;
        assert_eq!(Solution::maximize_sum(nums, k), 11);
    }
}
