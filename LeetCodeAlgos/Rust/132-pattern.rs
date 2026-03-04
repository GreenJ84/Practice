// Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].

// Return true if there is a 132 pattern in nums, otherwise, return false.

struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut num_k = i32::MIN;

        for &num in nums.iter().rev() {
            if num < num_k {
                return true; // num is num_i
            }
            while let Some(&top) = stack.last() {
                if num > top {
                    num_k = stack.pop().unwrap();
                } else {
                    break;
                }
            }
            stack.push(num);
        }

        false
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::find132pattern(nums), false);
  }

  #[test]
  fn test_2() {
    let nums = vec![3, 1, 4, 2];
    assert_eq!(Solution::find132pattern(nums), true);
  }

  #[test]
  fn test_3() {
    let nums = vec![-1, 3, 2, 0];
    assert_eq!(Solution::find132pattern(nums), true);
  }

  #[test]
  fn test_4() {
    let nums = vec![1];
    assert_eq!(Solution::find132pattern(nums), false);
  }
}