// Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.

// A shift on s consists of moving the leftmost character of s to the rightmost position.

// For example, if s = "abcde", then it will be "bcdea" after one shift.

struct Solution;
impl Solution {
  pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
      return false;
    }
    (s.clone() + &s).contains(&goal)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()));
  }

  #[test]
  fn test_example_2() {
    assert!(!Solution::rotate_string("abcde".to_string(), "abced".to_string()));
  }

  #[test]
  fn test_single_character() {
    assert!(Solution::rotate_string("a".to_string(), "a".to_string()));
  }

  #[test]
  fn test_no_match() {
    assert!(!Solution::rotate_string("ab".to_string(), "bc".to_string()));
  }

  #[test]
  fn test_different_lengths() {
    assert!(!Solution::rotate_string("abc".to_string(), "ab".to_string()));
  }

  #[test]
  fn test_all_same_chars() {
    assert!(Solution::rotate_string("aaaa".to_string(), "aaaa".to_string()));
  }

  #[test]
  fn test_full_rotation() {
    assert!(Solution::rotate_string("abc".to_string(), "abc".to_string()));
  }

  #[test]
  fn test_one_shift() {
    assert!(Solution::rotate_string("abc".to_string(), "bca".to_string()));
  }
}