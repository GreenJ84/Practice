// Given a binary array nums, return the maximum number of consecutive 1's in the array.

struct Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0i32;
        let mut run = 0i32;

        for num in &nums {
          match num {
            1 => {
              run += 1;
            },
            _ => {
              if run > ans { ans = run; }
              if run > 0 { run = 0; }
            }
          }
        }
        ans.max(run)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
  }

  #[test]
  fn test_all_ones() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 1, 1]), 4);
  }

  #[test]
  fn test_single_one() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
  }

  #[test]
  fn test_single_zero() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![0]), 0);
  }

  #[test]
  fn test_all_zeros() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0, 0]), 0);
  }

  #[test]
  fn test_ones_at_end() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0, 1, 1, 1]), 3);
  }

  #[test]
  fn test_ones_at_start() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 1, 0, 0]), 3);
  }
}

