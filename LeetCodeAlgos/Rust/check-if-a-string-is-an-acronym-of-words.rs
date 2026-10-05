// Given an array of strings words and a string s, determine if s is an acronym of words.

// The string s is considered an acronym of words if it can be formed by concatenating the first character of each string in words in order. For example, "ab" can be formed from ["apple", "banana"], but it can't be formed from ["bear", "aardvark"].

// Return true if s is an acronym of words, and false otherwise.

// Constraints:
// 1 <= words.length <= 100
// 1 <= words[i].length <= 10
// 1 <= s.length <= 100
// words[i] and s consist of lowercase English letters.

struct Solution;
impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }
        for (i, ch) in s.chars().enumerate() {
            if ch != words[i].chars().next().unwrap() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ];
        let s = "abc".to_string();
        assert_eq!(Solution::is_acronym(words, s), true);
    }

    #[test]
    fn test_2() {
        let words = vec!["an".to_string(), "apple".to_string()];
        let s = "a".to_string();
        assert_eq!(Solution::is_acronym(words, s), false);
    }

    #[test]
    fn test_3() {
        let words = vec![
            "never".to_string(),
            "gonna".to_string(),
            "give".to_string(),
            "up".to_string(),
            "on".to_string(),
            "you".to_string(),
        ];
        let s = "ngguoy".to_string();
        assert_eq!(Solution::is_acronym(words, s), true);
    }
}
