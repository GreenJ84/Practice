// You are given a string s consisting of lowercase English letters.

// The score of a string is the sum of the positions of its characters in the alphabet, where 'a' = 1, 'b' = 2, ..., 'z' = 26.

// Determine whether there exists an index i such that the string can be split into two non-empty substrings s[0..i] and s[(i + 1)..(n - 1)] that have equal scores.

// Return true if such a split exists, otherwise return false.

struct Solution;
impl Solution {
  pub fn score_balance(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();

    let eval_char = |ch: &char| *ch as i32 - 96 ;
    let (mut left, mut l_sum) = (0usize, eval_char(&s[0]));
    let (mut right, mut r_sum) = (s.len() - 1, eval_char(&s[s.len() - 1]));
    while right - left > 1 {
      if l_sum < r_sum {
        left += 1;
        l_sum += eval_char(&s[left])
      } else {
        right -= 1;
        r_sum += eval_char(&s[right])
      }
    }

    let diff = l_sum - r_sum;

    return diff == 0 ||
    right < s.len() - 1 && l_sum - diff == eval_char(&s[right]) ||
    left > 0 && r_sum + diff == eval_char(&s[left]);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::score_balance("adcb".to_string()), true);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::score_balance("bace".to_string()), false);
  }

    #[test]
  fn test_example_3() {
    assert_eq!(Solution::score_balance("caa".to_string()), false);
  }

  #[test]
  fn test_minimum_length() {
    assert_eq!(Solution::score_balance("ab".to_string()), false);
  }

  #[test]
  fn test_equal_characters() {
    assert_eq!(Solution::score_balance("aa".to_string()), true);
  }

  #[test]
  fn test_all_same_char() {
    assert_eq!(Solution::score_balance("aaaa".to_string()), true);
  }

  #[test]
  fn test_single_split_point() {
    assert_eq!(Solution::score_balance("abc".to_string()), true);
  }

  #[test]
  fn test_balanced_middle() {
    assert_eq!(Solution::score_balance("acdb".to_string()), false);
  }

  #[test]
  fn test_no_balance() {
    assert_eq!(Solution::score_balance("zzza".to_string()), false);
  }

  #[test]
  fn test_long_string() {
    assert_eq!(Solution::score_balance("abcd".to_string()), false);
  }

  #[test]
  fn test_alternating() {
    assert_eq!(Solution::score_balance("abab".to_string()), true);
  }
}
