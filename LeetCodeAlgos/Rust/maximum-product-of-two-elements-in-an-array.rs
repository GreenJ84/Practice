// Given the array of integers nums, you will choose two different indices i and j of that array. Return the maximum value of (nums[i]-1)*(nums[j]-1).

// Constraints:
// 2 <= nums.length <= 500
// 1 <= nums[i] <= 10^3

struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut first, mut second) = if nums[0] >= nums[1] {
          (nums[0], nums[1])
        } else {
          (nums[1], nums[0])
        };
        for i in 2..nums.len() {
          match nums[i] {
            x if x > first => {
              second = first;
              first = x;
            },
            x if x > second => {
              second = x;
            },
            _ => {}
          }
        }
        (first-1) * (second-1)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let nums = vec![3, 4, 5, 2];
    let result = Solution::max_product(nums);
    assert_eq!(result, 12);
  }

  #[test]
  fn test_2() {
    let nums = vec![1, 5, 4, 5];
    let result = Solution::max_product(nums);
    assert_eq!(result, 16);
  }
}

// Example 1:

// Input: nums = [3,4,5,2]
// Output: 12 
// Explanation: If you choose the indices i=1 and j=2 (indexed from 0), you will get the maximum value, that is, (nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12. 
// Example 2:

// Input: nums = [1,5,4,5]
// Output: 16
// Explanation: Choosing the indices i=1 and j=3 (indexed from 0), you will get the maximum value of (5-1)*(5-1) = 16.
// Example 3:

// Input: nums = [3,7]
// Output: 12
