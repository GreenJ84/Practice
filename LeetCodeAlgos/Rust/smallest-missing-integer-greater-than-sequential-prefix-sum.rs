// You are given a 0-indexed array of integers nums.

// A prefix nums[0..i] is sequential if, for all 1 <= j <= i, nums[j] = nums[j - 1] + 1. In particular, the prefix consisting only of nums[0] is sequential.

// Return the smallest integer x missing from nums such that x is greater than or equal to the sum of the longest sequential prefix.

// Constraints:
// 1 <= nums.length <= 50
// 1 <= nums[i] <= 50

struct Solution;
impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];

        let mut prefix = true;
        let mut found = vec![false; 51];
        found[sum as usize] = true;
        for j in 1..nums.len() {
            if prefix && nums[j] == nums[j - 1] + 1 {
                sum += nums[j];
            } else if prefix {
                prefix = false;
            }
            found[nums[j] as usize] = true;
        }
        while sum < 51 && found[sum as usize] {
            sum += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 2, 5];
        let result = Solution::missing_integer(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 4, 5, 1, 12, 14, 13];
        let result = Solution::missing_integer(nums);
        assert_eq!(result, 15);
    }
}
