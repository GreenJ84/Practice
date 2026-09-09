// You are given an array of positive integers nums.

// You have to check if it is possible to select two or more elements in the array such that the bitwise OR of the selected elements has at least one trailing zero in its binary representation.

// For example, the binary representation of 5, which is "101", does not have any trailing zeros, whereas the binary representation of 4, which is "100", has two trailing zeros.

// Return true if it is possible to select two or more elements whose bitwise OR has trailing zeros, return false otherwise.

// Constraints:
// 2 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut even = false;
        for num in nums {
            if num % 2 == 0 {
                if even {
                    return true;
                }
                even = true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let nums = vec![1,2,3,4,5];
        let result = Solution::has_trailing_zeros(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![2,4,8,16];
        let result = Solution::has_trailing_zeros(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let nums = vec![1,3,5,7,9];
        let result = Solution::has_trailing_zeros(nums);
        assert_eq!(result, false);
    }
}
