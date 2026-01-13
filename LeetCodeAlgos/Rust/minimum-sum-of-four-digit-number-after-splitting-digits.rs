// You are given a positive integer num consisting of exactly four digits. Split num into two new integers new1 and new2 by using the digits found in num. Leading zeros are allowed in new1 and new2, and all the digits found in num must be used.

// For example, given num = 2932, you have the following digits: two 2's, one 9 and one 3. Some of the possible pairs [new1, new2] are [22, 93], [23, 92], [223, 9] and [2, 329].
// Return the minimum possible sum of new1 and new2.

struct Solution;
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut n = num;
        let mut digits = Vec::new();

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.sort();
        for idx in 0..=3 {
            n += if idx < 2 {
                digits[idx] * 10
            } else {
                digits[idx]
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::minimum_sum(2932), 52);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::minimum_sum(4009), 13);
  }

  #[test]
  fn test_minimum_number() {
    assert_eq!(Solution::minimum_sum(1000), 1);
  }

  #[test]
  fn test_maximum_number() {
    assert_eq!(Solution::minimum_sum(9999), 198);
  }

  #[test]
  fn test_all_same_digits() {
    assert_eq!(Solution::minimum_sum(1111), 22);
  }

  #[test]
  fn test_sorted_ascending() {
    assert_eq!(Solution::minimum_sum(1234), 37);
  }

  #[test]
  fn test_sorted_descending() {
    assert_eq!(Solution::minimum_sum(4321), 37);
  }

  #[test]
  fn test_with_zeros() {
    assert_eq!(Solution::minimum_sum(1020), 3);
  }

  #[test]
  fn test_two_pairs() {
    assert_eq!(Solution::minimum_sum(5678), 125);
  }
}
