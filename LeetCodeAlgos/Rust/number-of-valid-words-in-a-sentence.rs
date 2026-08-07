// A sentence consists of lowercase letters ('a' to 'z'), digits ('0' to '9'), hyphens ('-'), punctuation marks ('!', '.', and ','), and spaces (' ') only. Each sentence can be broken down into one or more tokens separated by one or more spaces ' '.

// A token is a valid word if all three of the following are true:

// It only contains lowercase letters, hyphens, and/or punctuation (no digits).
// There is at most one hyphen '-'. If present, it must be surrounded by lowercase characters ("a-b" is valid, but "-ab" and "ab-" are not valid).
// There is at most one punctuation mark. If present, it must be at the end of the token ("ab,", "cd!", and "." are valid, but "a!b" and "c.," are not valid).
// Examples of valid words include "a-b.", "afad", "ba-c", "a!", and "!".

// Given a string sentence, return the number of valid words in sentence.

// Constraints:
// 1 <= sentence.length <= 1000
// sentence only contains lowercase English letters, digits, ' ', '-', '!', '.', and ','.
// There will be at least 1 token.

struct Solution;
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let words = sentence
            .split_whitespace()
            .map(|w| w.as_bytes())
            .collect::<Vec<&[u8]>>();

        let mut ans = 0;
        for &word in &words {
            let mut hyphens = 0;
            let n = word.len();
            for i in 0..n {
                match word[i] {
                    b'0'..=b'9' => {
                        ans -= 1;
                        break;
                    }
                    b'-' => {
                        if i == 0
                            || !word[i - 1].is_ascii_lowercase()
                            || i == n - 1
                            || !word[i + 1].is_ascii_lowercase()
                            || hyphens == 1
                        {
                            ans -= 1;
                            break;
                        }

                        hyphens += 1;
                    }
                    b'!' | b'.' | b',' => {
                        if i != n - 1 {
                            ans -= 1;
                            break;
                        }
                    }
                    _ => {}
                }
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
    fn test_1() {
        let sentence = "cat and  dog".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let sentence = "!this  1-s b8d!".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let sentence = "alice and  bob are playing stone-game10".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_4() {
        let sentence = "alice-.".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_5() {
        let sentence = "9".to_string();
        let result = Solution::count_valid_words(sentence);
        assert_eq!(result, 0);
    }
}
