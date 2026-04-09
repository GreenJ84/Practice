// You are given a 0-indexed string array words, where words[i] consists of lowercase English letters.

// In one operation, select any index i such that 0 < i < words.length and words[i - 1] and words[i] are anagrams, and delete words[i] from words. Keep performing this operation as long as you can select an index that satisfies the conditions.

// Return words after performing all operations. It can be shown that selecting the indices for each operation in any arbitrary order will lead to the same result.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase using all the original letters exactly once. For example, "dacb" is an anagram of "abdc".

struct Solution;

use std::collections::VecDeque;
impl Solution {
    fn freq_count(word: &str) -> [u8; 26] {
        let mut count = [0; 26];
        for c in word.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        count
    }

    fn arr_equal(arr1: &[u8; 26], arr2: &[u8; 26]) -> bool {
        for i in 0..26 {
            if arr1[i] != arr2[i] {
                return false;
            }
        }
        true
    }

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut words = words.iter();
        let mut prev_freq = VecDeque::new();
        ans.push(words.next().unwrap().to_string());
        prev_freq.push_back(Self::freq_count(&ans[0]));
        for word in words {
            let freq = Self::freq_count(&word);
            if !Self::arr_equal(&freq, &prev_freq.back().unwrap()) {
                ans.push(word.to_string());
                prev_freq.push_back(freq);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["abba".to_string(), "baba".to_string(), "bbaa".to_string(), "cd".to_string(), "cd".to_string()];
        let expected = vec!["abba".to_string(), "cd".to_string()];
        assert_eq!(Solution::remove_anagrams(words), expected);
    }

    #[test]
    fn test_2() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
        assert_eq!(Solution::remove_anagrams(words), expected);
    }
}
