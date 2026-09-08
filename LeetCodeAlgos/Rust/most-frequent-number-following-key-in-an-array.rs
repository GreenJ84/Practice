// You are given a 0-indexed integer array nums. You are also given an integer key, which is present in nums.

// For every unique integer target in nums, count the number of times target immediately follows an occurrence of key in nums. In other words, count the number of indices i such that:

// 0 <= i <= nums.length - 2,
// nums[i] == key and,
// nums[i + 1] == target.
// Return the target with the maximum count. The test cases will be generated such that the target with maximum count is unique.

// Constraints:
// 2 <= nums.length <= 1000
// 1 <= nums[i] <= 1000
// The test cases will be generated such that the answer is unique.

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = HashMap::<i32, i16>::new();
        for i in 0..nums.len() - 1 {
            if nums[i] == key{
                *map.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }
        map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1,100,200,1,100];
        let key = 1;
        let result = Solution::most_frequent(nums, key);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_2() {
        let nums = vec![2,2,2,2,3];
        let key = 2;
        let result = Solution::most_frequent(nums, key);
        assert_eq!(result, 2);
    }
}
