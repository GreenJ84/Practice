// You are given a string array words and a string s, where words[i] and s comprise only of lowercase English letters.

// Return the number of strings in words that are a prefix of s.

// A prefix of a string is a substring that occurs at the beginning of the string. A substring is a contiguous sequence of characters within a string.

struct Solution;
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
      words.iter()
        .filter(|w| s.starts_with(w) )
        .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        let s = "abc".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 3);
    }

    #[test]
    fn test_2() {
        let words = vec!["a".to_string(), "a".to_string()];
        let s = "aa".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 2);
    }
}

// Example 1:

// Input: words = ["a","b","c","ab","bc","abc"], s = "abc"
// Output: 3
// Explanation:
// The strings in words which are a prefix of s = "abc" are:
// "a", "ab", and "abc".
// Thus the number of strings in words which are a prefix of s is 3.
// Example 2:

// Input: words = ["a","a"], s = "aa"
// Output: 2
// Explanation:
// Both of the strings are a prefix of s.
// Note that the same string can occur multiple times in words, and it should be counted each time.
