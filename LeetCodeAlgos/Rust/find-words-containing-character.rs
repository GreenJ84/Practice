// You are given a 0-indexed array of strings words and a character x.

// Return an array of indices representing the words that contain the character x.

// Note that the returned array may be in any order.

// Constraints:
// 1 <= words.length <= 50
// 1 <= words[i].length <= 50
// x is a lowercase English letter.
// words[i] consists only of lowercase English letters.

struct Solution;
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| w.contains(x).then(|| i as i32))
            .collect::<Vec<i32>>()
    }

    pub fn find_words_containing1(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, w)| {
                if w.chars().any(|ch| ch == x) {
                    return Some(i as i32);
                }
                None
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["leet".to_string(), "code".to_string()];
        let x = 'e';
        let result = Solution::find_words_containing(words, x);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let x = 'a';
        let result = Solution::find_words_containing(words, x);
        assert_eq!(result, vec![0, 2]);
    }
}
