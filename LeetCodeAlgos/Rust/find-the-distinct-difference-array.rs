// You are given a 0-indexed array nums of length n.

// The distinct difference array of nums is an array diff of length n such that diff[i] is equal to the number of distinct elements in the suffix nums[i + 1, ..., n - 1] subtracted from the number of distinct elements in the prefix nums[0, ..., i].

// Return the distinct difference array of nums.

// Note that nums[i, ..., j] denotes the subarray of nums starting at index i and ending at index j inclusive. Particularly, if i > j then nums[i, ..., j] denotes an empty subarray.

// Constraints:
// 1 <= n == nums.length <= 50
// 1 <= nums[i] <= 50

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = HashSet::new();
        let mut suffix = HashSet::new();
        let mut ans =vec![0; nums.len()];
        for i in 0..nums.len() {
            prefix.insert(nums[i]);
            ans[i] += prefix.len() as i32;
            let end = nums.len() - 1 - i;
            ans[end] -= suffix.len() as i32;
            suffix.insert(nums[end]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let expected1 = vec![-3, -1, 1, 3, 5];
        assert_eq!(Solution::distinct_difference_array(nums1), expected1);

    }

    #[test]
    fn test_2() {
        let nums2 = vec![3, 2, 3, 4, 2];
        let expected2 = vec![-2, -1, 0, 2, 3];
        assert_eq!(Solution::distinct_difference_array(nums2), expected2);
    }
}
