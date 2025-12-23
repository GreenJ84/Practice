// You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).

// You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.

// Return the reformatted string or return an empty string if it is impossible to reformat the string.

struct Solution;
impl Solution {
    pub fn reformat(s: String) -> String {
        let (letters, digits) = s.chars().fold(
            (Vec::new(), Vec::new()),
            |(mut letters, mut digits), c| {
                if c.is_numeric() {
                    digits.push(c);
                } else {
                    letters.push(c);
                }
                (letters, digits)
            },
        );
        if (letters.len() as isize - digits.len() as isize).abs() > 1 {
            return String::new();
        }
        return if letters.len() > digits.len() {
            letters
                .into_iter()
                .zip(digits.into_iter().chain(std::iter::repeat(' ')))
                .flat_map(|(l, d)| [l, d])
                .filter(|&c| c != ' ')
                .collect()
        } else {
            digits
              .into_iter()
              .zip(letters.into_iter().chain(std::iter::repeat(' ')))
              .flat_map(|(d, l)| [d, l])
              .filter(|&c| c != ' ')
              .collect()
        };
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let result = Solution::reformat("a0b1c2".to_string());
    assert!(is_valid(&result));
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn test_example_2() {
    let result = Solution::reformat("leetcode".to_string());
    assert_eq!(result, "");
  }

  #[test]
  fn test_example_3() {
    let result = Solution::reformat("1229857369".to_string());
    assert_eq!(result, "");
  }

  #[test]
  fn test_single_letter() {
    let result = Solution::reformat("a".to_string());
    assert_eq!(result, "a");
  }

  #[test]
  fn test_single_digit() {
    let result = Solution::reformat("1".to_string());
    assert_eq!(result, "1");
  }

  #[test]
  fn test_alternating_letter_digit() {
    let result = Solution::reformat("a1b2".to_string());
    assert!(is_valid(&result));
    assert_eq!(result.len(), 4);
  }

  #[test]
  fn test_more_digits_than_letters() {
    let result = Solution::reformat("1a2b3c4".to_string());
    assert!(is_valid(&result));
    assert_eq!(result, "1a2b3c4");
    assert_eq!(result.len(), 7);
  }

  #[test]
  fn test_more_letters_than_digits() {
    let result = Solution::reformat("a1b2c".to_string());
    assert_eq!(result, "a1b2c");
  }

  fn is_valid(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
      let curr_is_digit = chars[i].is_numeric();
      let next_is_digit = chars[i + 1].is_numeric();
      if curr_is_digit == next_is_digit {
        return false;
      }
    }
    true
  }
}