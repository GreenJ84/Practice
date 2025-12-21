// Given an integer array nums, return the most frequent even element.

// If there is a tie, return the smallest one. If there is no such element, return -1.

struct Solution;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
      let mut even = std::collections::HashMap::<i32, usize>::new();

      nums.iter().for_each(|n| if *n % 2 == 0 {
        *even.entry(*n).or_default() += 1;
      });

      even.into_iter()
        .max_by(|(num1, count1), (num2, count2)|
          count1.cmp(count2).then(num2.cmp(num1))
        )
        .map(|(n, _)| n)
        .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::most_frequent_even(vec![0,1,2,2,4,4,1]), 2);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::most_frequent_even(vec![4,4,4,9,2,4]), 4);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::most_frequent_even(vec![29,47,21,41,13,37,25,7]), -1);
  }

  #[test]
  fn test_single_even_element() {
    assert_eq!(Solution::most_frequent_even(vec![2]), 2);
  }

  #[test]
  fn test_single_odd_element() {
    assert_eq!(Solution::most_frequent_even(vec![1]), -1);
  }

  #[test]
  fn test_all_even() {
    assert_eq!(Solution::most_frequent_even(vec![2,4,6,8,10]), 2);
  }

  #[test]
  fn test_all_odd() {
    assert_eq!(Solution::most_frequent_even(vec![1,3,5,7,9]), -1);
  }

  #[test]
  fn test_zero_is_most_frequent() {
    assert_eq!(Solution::most_frequent_even(vec![0,0,0,1,2]), 0);
  }

  #[test]
  fn test_tie_returns_smallest() {
    assert_eq!(Solution::most_frequent_even(vec![2,2,4,4]), 2);
  }

  #[test]
  fn test_large_numbers() {
    assert_eq!(Solution::most_frequent_even(vec![100000,100000,99999]), 100000);
  }
}