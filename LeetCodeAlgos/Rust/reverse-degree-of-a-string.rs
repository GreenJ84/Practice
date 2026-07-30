// Given a string s, calculate its reverse degree.

// The reverse degree is calculated as follows:

// For each character, multiply its position in the reversed alphabet ('a' = 26, 'b' = 25, ..., 'z' = 1) with its position in the string (1-indexed).
// Sum these products for all characters in the string.
// Return the reverse degree of s.

// Constraints:
// 1 <= s.length <= 1000
// s contains only lowercase English letters.

struct Solution;
impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut degree = 0;
        let mut idx = 1;
        for ch in s.chars() {
          degree += idx * (ch as i32 - 97 - 26).abs();
          idx += 1;
        }
        degree
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_1() {
        let s = "abc".to_string();
        let result = Solution::reverse_degree(s);
        assert_eq!(result, 148);
    }

    #[test]
    fn test_2() {
        let s = "zaza".to_string();
        let result = Solution::reverse_degree(s);
        assert_eq!(result, 160);
    }
}