// You are given a string s consisting of lowercase English letters ('a' to 'z').

// Your task is to:

// Find the vowel (one of 'a', 'e', 'i', 'o', or 'u') with the maximum frequency.
// Find the consonant (all other letters excluding vowels) with the maximum frequency.
// Return the sum of the two frequencies.

// Note: If multiple vowels or consonants have the same maximum frequency, you may choose any one of them. If there are no vowels or no consonants in the string, consider their frequency as 0.

// The frequency of a letter x is the number of times it occurs in the string.

// Constraints:
// 1 <= s.length <= 100
// s consists of lowercase English letters only.

struct Solution;
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut bytes = vec![0i32; 26];
        for b in s.as_bytes() {
            bytes[(b - b'a') as usize] += 1;
        }
        let mut v = 0;
        let mut c = 0;
        for idx in 0..=25 {
            match idx {
                0 | 4 | 8 | 14 | 20 => {
                    v = v.max(bytes[idx]);
                }
                _ => {
                    c = c.max(bytes[idx]);
                }
            }
        }
        v + c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "successes".to_string();
        let expected = 6;
        assert_eq!(Solution::max_freq_sum(s), expected);
    }

    #[test]
    fn test_2() {
        let s = "aeiaeia".to_string();
        let expected = 3;
        assert_eq!(Solution::max_freq_sum(s), expected);
    }

    #[test]
    fn test_3() {
        let s = "zz".to_string();
        let expected = 2;
        assert_eq!(Solution::max_freq_sum(s), expected);
    }
}
