// You are given an integer array nums.

// You are allowed to delete any number of elements from nums without making it empty. After performing the deletions, select a subarray of nums such that:

// All elements in the subarray are unique.
// The sum of the elements in the subarray is maximized.
// Return the maximum sum of such a subarray.

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::<i32>::new();
        let mut ans = 0i32;
        let mut max_seen = i32::MIN;

        for num in nums {
            max_seen = max_seen.max(num);
            if num >= 0 && !seen.contains(&num) {
                seen.insert(num);
                ans += num;
            }
        }
        return if max_seen >= 0 { ans } else { max_seen };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_sum(vec![1, 1, 0, 1, 1]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_sum(vec![42]), 42);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(Solution::max_sum(vec![-5]), -5);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(Solution::max_sum(vec![-1, -2, -3, -4]), -1);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::max_sum(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_duplicates_with_negatives() {
        assert_eq!(Solution::max_sum(vec![5, 5, -10, 3, 3]), 8);
    }

    #[test]
    fn test_mixed_values() {
        assert_eq!(Solution::max_sum(vec![10, -5, 10, 20, -5]), 30);
    }

    #[test]
    fn test_large_positive_with_duplicate() {
        assert_eq!(Solution::max_sum(vec![100, 100, 50]), 150);
    }
}
