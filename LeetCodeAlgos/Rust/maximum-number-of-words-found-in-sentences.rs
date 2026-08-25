// A sentence is a list of words that are separated by a single space with no leading or trailing spaces.

// You are given an array of strings sentences, where each sentences[i] represents a single sentence.

// Return the maximum number of words that appear in a single sentence.

// Constraints:
// 1 <= sentences.length <= 100
// 1 <= sentences[i].length <= 100
// sentences[i] consists only of lowercase English letters and ' ' only.
// sentences[i] does not have leading or trailing spaces.
// All the words in sentences[i] are separated by a single space.

struct Solution;
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.split_whitespace().count() as i32)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sen = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        let expected = 6;
        assert_eq!(Solution::most_words_found(sen), expected);
    }

    #[test]
    fn test_2() {
        let sen = vec![
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string(),
        ];
        let expected = 3;
        assert_eq!(Solution::most_words_found(sen), expected);
    }
}
