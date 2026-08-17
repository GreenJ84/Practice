// You are given a positive integer array nums and an integer k.

// Choose at most k elements from nums so that their sum is maximized. However, the chosen numbers must be distinct.

// Return an array containing the chosen numbers in strictly descending order.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 109
// 1 <= k <= nums.length

struct Solution;
impl Solution {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable_by(|a, b| b.cmp(&a));
        let mut ans = Vec::from([nums[0]]);
        if k == 1 {
            return ans;
        }
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            }
            ans.push(nums[i]);
            if ans.len() == k as usize {
                return ans;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![84, 93, 100, 77, 90];
        let k = 3;
        let result = Solution::max_k_distinct(nums, k);
        assert_eq!(result, vec![100, 93, 90]);
    }

    #[test]
    fn test_2() {
        let nums = vec![84, 93, 100, 77, 93];
        let k = 3;
        let result = Solution::max_k_distinct(nums, k);
        assert_eq!(result, vec![100, 93, 84]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 1, 1, 2, 2, 2];
        let k = 6;
        let result = Solution::max_k_distinct(nums, k);
        assert_eq!(result, vec![2, 1]);
    }
}
