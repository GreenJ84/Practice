// You are given a string s.

// Your task is to remove all digits by doing this operation repeatedly:
// - Delete the first digit and the closest non-digit character to its left.

// Return the resulting string after removing all digits.

// Note that the operation cannot be performed on a digit that does not have any non-digit character to its left.

struct Solution;
impl Solution {
  pub fn clear_digits(s: String) -> String {
    if s.len() < 2 { return s; }

    let mut s = s.chars().collect::<Vec<char>>();
    let mut idx = 0;
    loop {
      if idx >= s.len() { break; }
      match s[idx] {
        // digits
        ch if ch.is_ascii_digit() => {
          if idx > 0 && s[idx - 1].is_ascii_alphabetic() {
            s.remove(idx - 1);
            s.remove(idx - 1);
            idx -= 1;
            continue;
          }
        },
        // non-digit character
        _ => {}
      }
      idx += 1;
    }
    s.into_iter().collect::<String>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::clear_digits("abc".to_string()), "abc");
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::clear_digits("cb34".to_string()), "");
  }

  #[test]
  fn test_single_char() {
    assert_eq!(Solution::clear_digits("a".to_string()), "a");
  }

  #[test]
  fn test_single_digit() {
    assert_eq!(Solution::clear_digits("1".to_string()), "1");
  }

  #[test]
  fn test_digit_first() {
    assert_eq!(Solution::clear_digits("1abc".to_string()), "1abc");
  }

  #[test]
  fn test_multiple_digits() {
    assert_eq!(Solution::clear_digits("abc123".to_string()), "");
  }

  #[test]
  fn test_alternating() {
    assert_eq!(Solution::clear_digits("a1b2c3".to_string()), "");
  }

  #[test]
  fn test_trailing_letters() {
    assert_eq!(Solution::clear_digits("ab12cd".to_string()), "cd");
  }

  #[test]
  fn test_consecutive_digits() {
    assert_eq!(Solution::clear_digits("abc12".to_string()), "a");
  }

  #[test]
  fn test_long_string() {
    assert_eq!(Solution::clear_digits("abcdef123".to_string()), "abc");
  }

  #[test]
  fn test_mixed_pattern() {
    assert_eq!(Solution::clear_digits("a1b2c3d".to_string()), "d");
  }
}