// Given an array of strings patterns and a string word, return the number of strings in patterns that exist as a substring in word.

// A substring is a contiguous sequence of characters within a string.

// Constraints:
// - 1 <= patterns.length <= 100
// - 1 <= patterns[i].length <= 100
// - 1 <= word.length <= 100
// - patterns[i] and word consist of lowercase English letters.

struct Solution;
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .into_iter()
            .filter(|pattern| word.contains(pattern))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let patterns = vec![
            "a".to_string(),
            "abc".to_string(),
            "bc".to_string(),
            "d".to_string(),
        ];
        let word = "abc".to_string();
        assert_eq!(Solution::num_of_strings(patterns, word), 3);
    }

    #[test]
    fn test_2() {
        let patterns = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let word = "aaaaabbbbb".to_string();
        assert_eq!(Solution::num_of_strings(patterns, word), 2);
    }

    #[test]
    fn test_3() {
        let patterns = vec!["a".to_string(), "a".to_string(), "a".to_string()];
        let word = "ab".to_string();
        assert_eq!(Solution::num_of_strings(patterns, word), 3);
    }
}
