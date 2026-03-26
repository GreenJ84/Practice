// You are given 2 integer arrays nums1 and nums2 of lengths n and m respectively. You are also given a positive integer k.

// A pair (i, j) is called good if nums1[i] is divisible by nums2[j] * k (0 <= i <= n - 1, 0 <= j <= m - 1).

// Return the total number of good pairs.

struct Solution;
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] % (nums2[j] * k) == 0 {
                    ans += 1;
                }
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
        let nums1 = vec![1, 3, 4];
        let nums2 = vec![1, 3, 4];
        let k = 1;
        assert_eq!(Solution::number_of_pairs(nums1, nums2, k), 5);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 4, 12];
        let nums2 = vec![2, 4];
        let k = 3;
        assert_eq!(Solution::number_of_pairs(nums1, nums2, k), 2);
    }
}
