// You are given an integer array nums.

// Return true if the frequency of any element of the array is prime, otherwise, return false.

// The frequency of an element x is the number of times it occurs in the array.

// A prime number is a natural number greater than 1 with only two factors, 1 and itself.

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
      let mut freq = HashMap::<&i32, i32>::new();
      for num in &nums {
          freq.entry(num)
            .and_modify(|x| *x += 1 )
            .or_insert(1);
      }

      let primes: HashSet<i32> = HashSet::from([
          2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
          73, 79, 83, 89, 97e
      ]);
      for val in freq.into_values() {
        if primes.contains(&val) { 
          return true;
        }
      }
      false
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]), true);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]), false);
  }

  #[test]
  fn example_3() {
    assert_eq!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]), true);
  }

  #[test]
  fn single_element() {
    assert_eq!(Solution::check_prime_frequency(vec![1]), false);
  }

  #[test]
  fn all_same_prime_frequency() {
    assert_eq!(Solution::check_prime_frequency(vec![5, 5, 5]), true);
  }

  #[test]
  fn frequency_of_2() {
    assert_eq!(Solution::check_prime_frequency(vec![7, 7, 8]), true);
  }

  #[test]
  fn frequency_of_4_composite() {
    assert_eq!(Solution::check_prime_frequency(vec![1, 1, 1, 1]), false);
  }

  #[test]
  fn frequency_of_5() {
    assert_eq!(Solution::check_prime_frequency(vec![0, 0, 0, 0, 0]), true);
  }

  #[test]
  fn multiple_elements_no_prime_frequency() {
    assert_eq!(Solution::check_prime_frequency(vec![1, 1, 1, 1, 2, 2, 2, 2]), false);
  }

  #[test]
  fn max_length_array() {
    let nums = vec![1; 100];
    assert_eq!(Solution::check_prime_frequency(nums), false);
  }
}