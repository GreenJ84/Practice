// You are given two strings s and t such that every character occurs at most once in s and t is a permutation of s.

// The permutation difference between s and t is defined as the sum of the absolute difference between the index of the occurrence of each character in s and the index of the occurrence of the same character in t.

// Return the permutation difference between s and t.

// Constraints:
// 1 <= s.length <= 26
// Each character occurs at most once in s.
// t is a permutation of s.
// s consists only of lowercase English letters.

struct Solution;
impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut pos = vec![0; 26];
        for (idx, b) in s.bytes().enumerate() {
            pos[(b - b'a') as usize] = idx as i32;
        }
        let mut ans = 0;
        for (idx, b) in t.bytes().enumerate() {
            ans += (pos[(b - b'a') as usize] - idx as i32).abs()
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abc".to_string();
        let t = "bac".to_string();
        let result = Solution::find_permutation_difference(s, t);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let s = "abcde".to_string();
        let t = "edbac".to_string();
        let result = Solution::find_permutation_difference(s, t);
        assert_eq!(result, 12);
    }
}
