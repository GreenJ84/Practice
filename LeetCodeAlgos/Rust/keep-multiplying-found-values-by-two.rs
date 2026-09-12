// You are given an array of integers nums. You are also given an integer original which is the first number that needs to be searched for in nums.

// You then do the following steps:

// If original is found in nums, multiply it by two (i.e., set original = 2 * original).
// Otherwise, stop the process.
// Repeat this process with the new number as long as you keep finding the number.
// Return the final value of original.

// Constraints:
// 1 <= nums.length <= 1000
// 1 <= nums[i], original <= 1000

struct Solution;
impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_unstable();
        for num in nums {
            if num == original {
                original *= 2;
                continue;
            } else if num > original {
                break;
            }
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 3, 6, 1, 12];
        let original = 3;
        let result = Solution::find_final_value(nums, original);
        assert_eq!(result, 24);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 7, 9];
        let original = 4;
        let result = Solution::find_final_value(nums, original);
        assert_eq!(result, 4);
    }
}
