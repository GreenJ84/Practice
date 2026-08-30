// For a string sequence, a string word is k-repeating if word concatenated k times is a substring of sequence. The word's maximum k-repeating value is the highest value k where word is k-repeating in sequence. If word is not a substring of sequence, word's maximum k-repeating value is 0.

// Given strings sequence and word, return the maximum k-repeating value of word in sequence.

// Constraints:
// 1 <= sequence.length <= 100
// 1 <= word.length <= 100
// sequence and word contains only lowercase English letters.

struct Solution;
impl Solution {
    pub fn max_repeating1(sequence: String, word: String) -> i32 {
        let (seq, m) = (sequence.as_bytes(), sequence.len());
        let (word, n) = (word.as_bytes(), word.len());
        let mut max = 0i32;

        for s_idx in 0..seq.len() {
            if seq[s_idx] == word[0] {
                let mut k = if n == 1 { 1i32 } else { 0i32 };
                let mut point = 1;
                while s_idx + point < m && seq[s_idx + point] == word[point % n] {
                    if point % n == n - 1 {
                        k += 1;
                    }
                    point += 1;
                }
                max = max.max(k);
                if s_idx + point == m {
                    return max;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sequence = "ababc".to_string();
        let word = "ab".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let sequence = "ababc".to_string();
        let word = "ba".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let sequence = "ababc".to_string();
        let word = "ac".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let sequence = "aaaaaaa".to_string();
        let word = "a".to_string();
        let result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 7);
    }
}
