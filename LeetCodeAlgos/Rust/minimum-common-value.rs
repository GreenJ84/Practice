// Given two integer arrays nums1 and nums2, sorted in non-decreasing order, return the minimum integer common to both arrays. If there is no common integer amongst nums1 and nums2, return -1.

// Note that an integer is said to be common to nums1 and nums2 if both arrays have at least one occurrence of that integer.

// Constraints:
// 1 <= nums1.length, nums2.length <= 10^5
// 1 <= nums1[i], nums2[j] <= 10^9
// Both nums1 and nums2 are sorted in non-decreasing order.

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i1, mut i2) = (0usize, 0usize);
        while i1 < nums1.len() && i2 < nums2.len() {
            match nums1[i1].cmp(&nums2[i2]) {
                Ordering::Less => {
                    i1 += 1;
                }
                Ordering::Equal => {
                    return nums1[i1];
                }
                Ordering::Greater => {
                    i2 += 1;
                }
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
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];
        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 3, 6];
        let nums2 = vec![2, 3, 4, 5];
        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }
}
