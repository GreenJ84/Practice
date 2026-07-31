// An array is considered special if the parity of every pair of adjacent elements is different. In other words, one element in each pair must be even, and the other must be odd.

// You are given an array of integers nums. Return true if nums is a special array, otherwise, return false.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for idx in 1..nums.len() {
            if (nums[idx] + nums[idx - 1]) % 2 == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1];
        assert_eq!(Solution::is_array_special(nums), true);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 1, 4];
        assert_eq!(Solution::is_array_special(nums), true);
    }

    #[test]
    fn test_3() {
        let nums = vec![4, 3, 1, 6];
        assert_eq!(Solution::is_array_special(nums), false);
    }
}
