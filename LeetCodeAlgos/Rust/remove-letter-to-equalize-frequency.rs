// You are given a 0-indexed string word, consisting of lowercase English letters. You need to select one index and remove the letter at that index from word so that the frequency of every letter present in word is equal.

// Return true if it is possible to remove one letter so that the frequency of all letters in word are equal, and false otherwise.

// Note:

// The frequency of a letter x is the number of times it occurs in the string.
// You must remove exactly one letter and cannot choose to do nothing.

// Constraints:
// 2 <= word.length <= 100
// word consists of lowercase English letters only.

struct Solution;
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut freq = vec![0i32; 26];
        for ch in word.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }

        freq = freq.into_iter().filter(|f| *f != 0).collect();
        if freq.len() == 1 {
            return true;
        }

        let mut first = (freq[0], 1);
        let mut found = (0, 0);
        for i in 1..freq.len() {
            if first.0 == freq[i] {
                first.1 += 1;
            } else if found.0 == 0 {
                if freq[i] < first.0 {
                    found = (first.0, first.1);
                    first = (freq[i], 1);
                } else {
                    found = (freq[i], 1);
                }
            } else if found.0 == freq[i] {
                found.1 += 1;
            } else {
                return false;
            }
        }
        match (first, found) {
            ((1, 1), _) | ((1, _), (_, 0)) => true,
            ((f, _), (g, 1)) => f + 1 == g,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let word = "abcc".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }

    #[test]
    fn test_2() {
        let word = "aazz".to_string();
        assert_eq!(Solution::equal_frequency(word), false);
    }

    #[test]
    fn test_3() {
        let word = "abc".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }

    #[test]
    fn test_4() {
        let word = "cac".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }

    #[test]
    fn test_5() {
        let word = "bbac".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }

    #[test]
    fn test_6() {
        let word = "bbacc".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }

    #[test]
    fn test_7() {
        let word = "cccd".to_string();
        assert_eq!(Solution::equal_frequency(word), true);
    }
}
