// Given a 0-indexed integer array nums of length n and an integer k, return the number of pairs (i, j) where 0 <= i < j < n, such that nums[i] == nums[j] and (i * j) is divisible by k.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i], k <= 100

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut ans = 0;
        for (idx, val) in nums.iter().enumerate() {
            if let Some(entry) = map.get_mut(val) {
                for prev in entry.iter() {
                    if (prev * idx) % k == 0 {
                        ans += 1;
                    }
                }
                entry.push(idx);
            } else {
                map.insert(*val, Vec::from([idx]));
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
        let nums = vec![3, 1, 2, 2, 2, 1, 3];
        let k = 2;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 1;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, 0);
    }
}
