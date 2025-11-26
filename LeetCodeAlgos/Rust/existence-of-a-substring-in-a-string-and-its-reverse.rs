// Given a string s, find any substring of length 2 which is also present in the reverse of s.

// Return true if such a substring exists, and false otherwise.

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
  pub fn is_substring_present(s: String) -> bool {
    if s.len() < 2 { return false; }

    let chars = s.chars().collect::<Vec<char>>();
    let mut map = HashMap::<char, HashSet<char>>::new();

    for i in 0..chars.len() - 1 {
      if &chars[i] == &chars[i + 1] { return true; }
      map.entry(chars[i])
        .and_modify(|s| { s.insert(chars[i + 1]); })
        .or_insert( HashSet::from([chars[i + 1]; 1] ));
    }
    for i in (1..chars.len()).rev() {
      if let Some(expected) = map.get(&chars[i]) {
        if expected.contains(&chars[i - 1]) {
          return true;
        }
      }
    }
    false
  }

  // pub fn is_substring_present1(s: String) -> bool {
  //   if s.len() < 2 { return false; }
  //   let reversed: String = s.chars().rev().collect(); // 0(n)
  //   for i in 0..s.len() - 1 { // 0(n)
  //       let substring = &s[i..i + 2];
  //       if reversed.contains(substring) { // 0(n)
  //           return true;
  //       }
  //   }
  //   false
  // }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example_1() {
    assert_eq!(Solution::is_substring_present("leetcode".to_string()), true);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::is_substring_present("abcba".to_string()), true);
  }

  #[test]
  fn example_3() {
    assert_eq!(Solution::is_substring_present("abcd".to_string()), false);
  }

  #[test]
  fn empty_string() {
    assert_eq!(Solution::is_substring_present("".to_string()), false);
  }

  #[test]
  fn single_character() {
    assert_eq!(Solution::is_substring_present("x".to_string()), false);
  }

  #[test]
  fn two_same_characters() {
    assert_eq!(Solution::is_substring_present("aa".to_string()), true);
  }

  #[test]
  fn two_different_characters() {
    assert_eq!(Solution::is_substring_present("ab".to_string()), false);
  }

  #[test]
  fn simple_palindrome() {
    assert_eq!(Solution::is_substring_present("aba".to_string()), true);
  }

  #[test]
  fn overlapping_substrings() {
    assert_eq!(Solution::is_substring_present("aab".to_string()), true);
  }

  #[test]
  fn long_palindrome() {
    assert_eq!(
      Solution::is_substring_present("abcdefgfedcba".to_string()),
      true
    );
  }
}

