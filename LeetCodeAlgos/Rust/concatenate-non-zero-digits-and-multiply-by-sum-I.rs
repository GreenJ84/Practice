// You are given an integer n.

// Form a new integer x by concatenating all the non-zero digits of n in their original order. If there are no non-zero digits, x = 0.

// Let sum be the sum of digits in x.

// Return an integer representing the value of x * sum.

struct Solution;
impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut x: i64 = 0;
        let mut lvl: i64 = 1;

        let mut sum: i64 = 0;

        while n > 0 {
          let digit = n % 10;
          if digit != 0 {
            sum += digit as i64;
            x += digit as i64 * lvl;
            lvl *= 10;
          }
          n /= 10;
        }
        x * sum
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    assert_eq!(Solution::sum_and_multiply(10203004), 12340);
  }

  #[test]
  fn test_example2() {
    assert_eq!(Solution::sum_and_multiply(1000), 1);
  }

  #[test]
  fn test_single_digit() {
    assert_eq!(Solution::sum_and_multiply(5), 25);
  }

  #[test]
  fn test_all_zeros() {
    assert_eq!(Solution::sum_and_multiply(0), 0);
  }

  #[test]
  fn test_no_zeros() {
    assert_eq!(Solution::sum_and_multiply(123), 738);
  }

  #[test]
  fn test_large_number() {
    assert_eq!(Solution::sum_and_multiply(1000000000), 1);
  }

  #[test]
  fn test_mixed_digits() {
    assert_eq!(Solution::sum_and_multiply(102030), 738);
  }

  #[test]
  fn test_trailing_zeros() {
    assert_eq!(Solution::sum_and_multiply(1230), 738);
  }
}