// You are given an integer array nums.

// Return an integer denoting the first even integer (earliest by array index) that appears exactly once in nums. If no such integer exists, return -1.

// An integer x is considered even if it is divisible by 2.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn first_unique_even(mut nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums = nums.into_iter().filter(|&n| {
            if n % 2 == 0 {
                map.entry(n)
                  .and_modify(|v| *v += 1)
                  .or_insert(1);
                return true;
            }
            false
        }).collect();
        for n in nums {
            if map.get(&n).unwrap_or(&0) == &1 {
                return n;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 2, 5, 4, 6];
        let result = Solution::first_unique_even(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 4];
        let result = Solution::first_unique_even(nums);
        assert_eq!(result, -1);
    }
}
