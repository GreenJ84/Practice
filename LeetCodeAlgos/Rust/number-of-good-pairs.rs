// Given an array of integers nums, return the number of good pairs.

// A pair (i, j) is called good if nums[i] == nums[j] and i < j.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        let mut pairs = 0i32;
        for num in nums {
          let count = map.entry(num).or_insert(0);
          pairs += *count;
          *count += 1;
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let result = Solution::num_identical_pairs(nums);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_2() {
    let nums = vec![1, 1, 1, 1];
    let result = Solution::num_identical_pairs(nums);
    assert_eq!(result, 6);
  }

  #[test]
  fn test_3() {
    let nums = vec![1, 2, 3];
    let result = Solution::num_identical_pairs(nums);
    assert_eq!(result, 0);
  }
}

// Example 1:

// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
// Example 2:

// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
// Example 3:

// Input: nums = [1,2,3]
// Output: 0
