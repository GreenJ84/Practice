// Given an integer array nums that does not contain any zeros, find the largest positive integer k such that -k also exists in the array.

// Return the positive integer k. If there is no such integer, return -1.

use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut max: i32 = -1;
        let set: HashSet<i32> = nums.iter().cloned().collect();

        for i in nums {
            if set.contains(&-i) {
                max = max.max(i.abs() as i32);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_k() {
        assert_eq!(Solution::find_max_k(vec![-1,2,-3,3]), 3)
    }

    #[test]
    fn test_find_max_k2() {
        assert_eq!(Solution::find_max_k(vec![-1,10,6,7,-7,1]), 7)
    }

    #[test]
    fn test_find_max_k3() {
        assert_eq!(Solution::find_max_k(vec![-10,8,6,7,-2,-3]), -1)
    }

    #[test]
    fn test_find_max_k4() {
        assert_eq!(Solution::find_max_k(vec![1,-10,8,6,7,-2,-3,-1, -8]), 8)
    }

    #[test]
    fn test_find_max_k5() {
        assert_eq!(Solution::find_max_k(vec![-7,-10,8,6,7,-2,-3]), 7)
    }

    #[test]
    fn test_find_max_k6() {
        assert_eq!(Solution::find_max_k(vec![10,5,1,-4,-2,10,5,7,-3,-3]), -1)
    }
}