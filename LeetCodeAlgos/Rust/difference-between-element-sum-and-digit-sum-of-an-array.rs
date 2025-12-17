// You are given a positive integer array nums.

// The element sum is the sum of all the elements in nums.
// The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.
// Return the absolute difference between the element sum and digit sum of nums.

// Note that the absolute difference between two integers x and y is defined as |x - y|.

struct Solution;
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut abs_sum = 0;
        for num in nums {
          if num < 10 { continue; }
          abs_sum += (num - Self::digit_sum(num)).abs()
        }
        abs_sum
    }

    fn digit_sum(num: i32) -> i32 {
        let mut sum = 0;
        let mut n = num;
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
  fn test_example_1() {
    assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
  }

  #[test]
  fn test_single_element() {
    assert_eq!(Solution::difference_of_sum(vec![5]), 0);
  }

  #[test]
  fn test_two_digit_numbers() {
    assert_eq!(Solution::difference_of_sum(vec![10, 20, 30]), 54);
  }

  #[test]
  fn test_max_constraint() {
    assert_eq!(Solution::difference_of_sum(vec![2000]), 1998);
  }

  #[test]
  fn test_all_single_digits() {
    assert_eq!(Solution::difference_of_sum(vec![9, 8, 7, 6, 5]), 0);
  }

  #[test]
  fn test_mixed_sizes() {
    assert_eq!(Solution::difference_of_sum(vec![99, 1, 2]), 81);
  }
}
