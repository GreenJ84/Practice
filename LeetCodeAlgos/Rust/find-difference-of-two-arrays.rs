// Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:

// answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
// answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
// Note that the integers in the lists may be returned in any order.

use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cp1 = Vec::new();
        let mut cp2 = Vec::new();
        let m = nums1.len();
        let n = nums2.len();
        for i in 0..max(m, n) {
            if i < m{
                if !nums2.contains(&nums1[i]) && !cp1.contains(&nums1[i]) {
                    cp1.push(nums1[i]);
                }
            }
            if i < n{
                if !nums1.contains(&nums2[i]) && !cp2.contains(&nums2[i]) {
                    cp2.push(nums2[i]);
                }
            }
        }
        Vec::from([cp1, cp2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_difference() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![2,4,6]),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn test_find_difference2() {
        assert_eq!(
            Solution::find_difference(vec![1,2,3,3], vec![1,1,2,2]),
            vec![vec![3], vec![]]
        );
    }
}