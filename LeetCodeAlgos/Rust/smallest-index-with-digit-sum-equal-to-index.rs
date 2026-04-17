// You are given an integer array nums.

// Return the smallest index i such that the sum of the digits of nums[i] is equal to i.

// If no such index exists, return -1.

struct Solution;
impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (idx, num) in nums.into_iter().enumerate() {
          if Self::digit_sum(num) == idx as i32 {
            return idx as i32;
          }
        }
        -1
    }

    fn digit_sum(mut n: i32) -> i32{
      let mut sum = 0;
      while n > 0 {
        sum += n % 10;
        n /= 10;
      }
      sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 2];
        assert_eq!(Solution::smallest_index(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 10, 11];
        assert_eq!(Solution::smallest_index(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::smallest_index(nums), -1);
    }
}

// Example 1:

// Input: nums = [1,3,2]

// Output: 2

// Explanation:

// For nums[2] = 2, the sum of digits is 2, which is equal to index i = 2. Thus, the output is 2.
// Example 2:

// Input: nums = [1,10,11]

// Output: 1

// Explanation:

// For nums[1] = 10, the sum of digits is 1 + 0 = 1, which is equal to index i = 1.
// For nums[2] = 11, the sum of digits is 1 + 1 = 2, which is equal to index i = 2.
// Since index 1 is the smallest, the output is 1.
// Example 3:

// Input: nums = [1,2,3]

// Output: -1

// Explanation:

// Since no index satisfies the condition, the output is -1.