// There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

// Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.

struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_set: std::collections::HashSet<char> = broken_letters.chars().collect();

        let mut ans = 0;
        for word in text.split_whitespace() {
            if word.chars().any(|c| broken_set.contains(&c)) {
                continue;
            }
            ans += 1;
        }
        ans
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()), 1);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()), 1);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::can_be_typed_words("leet code".to_string(), "e".to_string()), 0);
  }

  #[test]
  fn no_broken_letters() {
    assert_eq!(Solution::can_be_typed_words("hello world".to_string(), "".to_string()), 2);
  }

  #[test]
  fn all_words_typeable() {
    assert_eq!(Solution::can_be_typed_words("abc def".to_string(), "xyz".to_string()), 2);
  }

  #[test]
  fn single_word() {
    assert_eq!(Solution::can_be_typed_words("hello".to_string(), "h".to_string()), 0);
  }

  #[test]
  fn single_word_typeable() {
    assert_eq!(Solution::can_be_typed_words("rust".to_string(), "xyz".to_string()), 1);
  }

  #[test]
  fn multiple_broken_letters() {
    assert_eq!(Solution::can_be_typed_words("the quick brown fox".to_string(), "qbf".to_string()), 1);
  }

  #[test]
  fn all_words_broken() {
    assert_eq!(Solution::can_be_typed_words("abc def ghi".to_string(), "abcdefghi".to_string()), 0);
  }
}
