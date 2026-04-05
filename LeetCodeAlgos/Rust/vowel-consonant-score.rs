// You are given a string s consisting of lowercase English letters, spaces, and digits.

// Let v be the number of vowels in s and c be the number of consonants in s.

// A vowel is one of the letters 'a', 'e', 'i', 'o', or 'u', while any other letter in the English alphabet is considered a consonant.

// The score of the string s is defined as follows:

// If c > 0, the score = floor(v / c) where floor denotes rounding down to the nearest integer.
// Otherwise, the score = 0.
// Return an integer denoting the score of the string.

struct Solution;
impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut v = 0;
        let mut c = 0;
        for ch in s.chars() {
          if ch.is_ascii_alphabetic() {
            if ['a','e','i','o','u'].contains(&ch) {
              v += 1;
            } else {
              c += 1;
            }
          }
        }
        if c == 0 { return 0; }
        v / c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "cooear".to_string();
        assert_eq!(Solution::vowel_consonant_score(s), 2);
    }

    #[test]
    fn test_example_2() {
        let s = "axeyizou".to_string();
        assert_eq!(Solution::vowel_consonant_score(s), 1);
    }

    #[test]
    fn test_example_3() {
        let s = "au 123".to_string();
        assert_eq!(Solution::vowel_consonant_score(s), 0);
    }
}
