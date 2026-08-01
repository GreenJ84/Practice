// Given two integer arrays nums1 and nums2, return the maximum length of a subarray that appears in both arrays.

struct Solution;
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut prev = vec![0; nums1.len() + 1];
        let mut lvl = vec![0];
        for y in 0..nums2.len() {
          for x in 0..nums1.len() {
            if nums1[x] == nums2[y] {
              let val = 1 + prev[x];
              lvl.push(val);
              if val > ans {
                ans = val;
              }
            } else {
              lvl.push(0);
            }
          }
          prev = lvl;
          lvl = vec![0];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 3, 2, 1];
        let nums2 = vec![3, 2, 1, 4, 7];
        assert_eq!(Solution::find_length(nums1, nums2), 3);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![0, 0, 0, 0, 0];
        assert_eq!(Solution::find_length(nums1, nums2), 5);
    }
}

// Example 1:

// Input: nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
// Output: 3
// Explanation: The repeated subarray with maximum length is [3,2,1].
// Example 2:

// Input: nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
// Output: 5
// Explanation: The repeated subarray with maximum length is [0,0,0,0,0].

// Constraints:

// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 100
