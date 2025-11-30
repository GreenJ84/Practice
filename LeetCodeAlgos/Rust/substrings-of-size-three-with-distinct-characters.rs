// A string is good if there are no repeated characters.

// Given a string s​​​​​, return the number of good substrings of length three in s​​​​​​.

// Note that if there are multiple occurrences of the same substring, every occurrence should be counted.

// A substring is a contiguous sequence of characters in a string.

struct Solution;

use std::collections::VecDeque;
impl Solution {
  pub fn count_good_substrings(s: String) -> i32 {
    if s.len() < 3 { return 0; }

    let mut window = VecDeque::<char>::with_capacity(3);
    let mut res = 0i32;
    for ch in s.chars() {
      if window.is_empty() { window.push_back(ch); continue; }
      if window.contains(&ch) {
        while let Some(o_ch) = window.pop_front() {
          if o_ch == ch { break; }
        }
      }
      window.push_back(ch);
      if window.len() == 3 {
        res += 1;
        window.pop_front();
      }
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example1() {
    assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
  }

  #[test]
  fn test_example2() {
    assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
  }

  #[test]
  fn test_too_short_one_char() {
    assert_eq!(Solution::count_good_substrings("a".to_string()), 0);
  }

  #[test]
  fn test_too_short_two_chars() {
    assert_eq!(Solution::count_good_substrings("ab".to_string()), 0);
  }

  #[test]
  fn test_exact_three_all_distinct() {
    assert_eq!(Solution::count_good_substrings("abc".to_string()), 1);
  }

  #[test]
  fn test_exact_three_with_repeat() {
    assert_eq!(Solution::count_good_substrings("aab".to_string()), 0);
  }

  #[test]
  fn test_overlapping_distinct_substrings() {
    // "abcdef" -> "abc","bcd","cde","def" all distinct
    assert_eq!(Solution::count_good_substrings("abcdef".to_string()), 4);
  }

  #[test]
  fn test_repeated_pattern_occurrences() {
    // "abcabcabc" has 7 length-3 substrings and each is letters-distinct
    assert_eq!(Solution::count_good_substrings("abcabcabc".to_string()), 7);
  }

  #[test]
  fn test_mixed_pattern() {
    // "abac" -> "aba"(not good), "bac"(good)
    assert_eq!(Solution::count_good_substrings("abac".to_string()), 1);
  }

  #[test]
  fn test_all_same_long() {
    let s = "z".repeat(100);
    assert_eq!(Solution::count_good_substrings(s), 0);
  }
}