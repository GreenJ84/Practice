// Given an binary array nums and an integer k, return true if all 1's are at least k places away from each other, otherwise return false.

// Constraints:
// 1 <= nums.length <= 10^5
// 0 <= k <= nums.length
// nums[i] is 0 or 1

struct Solution;
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last: Option<usize> = None;
        for (&r_idx, num) in nums.iter().enumerate() {
            if num == 1 {
                if let Some(l_idx) = last {
                    if r_idx - l_idx <= k as usize {
                        return false;
                    }
                }
                last = Some(r_idx);
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 0, 0, 0, 1, 0, 0, 1];
        let k: i32 = 2;
        let expected = true;
        assert_eq!(Solution::k_length_apart(nums, k), expected);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 0, 0, 1, 0, 1];
        let k: i32 = 2;
        let expected = false;
        assert_eq!(Solution::k_length_apart(nums, k), expected);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 1, 1, 1, 1];
        let k: i32 = 0;
        let expected = true;
        assert_eq!(Solution::k_length_apart(nums, k), expected);
    }
}
