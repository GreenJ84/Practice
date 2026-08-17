// You are given an integer array nums.

// Consider all pairs of distinct values x and y from nums such that:

// x < y
// x and y have different frequencies in nums.
// Among all such pairs:

// Choose the pair with the smallest possible value of x.
// If multiple pairs have the same x, choose the one with the smallest possible value of y.
// Return an integer array [x, y]. If no valid pair exists, return [-1, -1].

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut map = map.into_iter().collect::<Vec<(i32, i32)>>();
        map.sort_unstable_by_key(|p| p.0);
        for i in 1..map.len() {
            if map[i].1 != map[0].1 {
                return vec![map[0].0, map[i].0];
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
        let nums = vec![1, 1, 2, 2, 3, 4];
        let result = Solution::min_distinct_freq_pair(nums);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 5];
        let result = Solution::min_distinct_freq_pair(nums);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![7];
        let result = Solution::min_distinct_freq_pair(nums);
        assert_eq!(result, vec![-1, -1]);
    }
}
