// You are given three strings: s1, s2, and s3. In one operation you can choose one of these strings and delete its rightmost character. Note that you cannot completely empty a string.

// Return the minimum number of operations required to make the strings equal. If it is impossible to make them equal, return -1.

// Constraints:
// 1 <= s1.length, s2.length, s3.length <= 100
// s1, s2 and s3 consist only of lowercase English letters.

struct Solution;
impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut len = s1.len().min(s2.len()).min(s3.len());
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        for idx in 0..len {
            if s1[idx] != s2[idx] || s1[idx] != s3[idx] {
                if idx == 0 { return -1; }
                len = idx;
                break;
            }
        }

        (s1.len() + s2.len() + s3.len() - 3 * len) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s1 = "abc".to_string();
        let s2 = "abb".to_string();
        let s3 = "ab".to_string();
        assert_eq!(Solution::find_minimum_operations(s1, s2, s3), 2);
    }

    #[test]
    fn test_2() {
        let s1 = "dac".to_string();
        let s2 = "bac".to_string();
        let s3 = "cac".to_string();
        assert_eq!(Solution::find_minimum_operations(s1, s2, s3), -1);
    }
}

// Example 1:

// Input: s1 = "abc", s2 = "abb", s3 = "ab"

// Output: 2

// Explanation: Deleting the rightmost character from both s1 and s2 will result in three equal strings.

// Example 2:

// Input: s1 = "dac", s2 = "bac", s3 = "cac"

// Output: -1

// Explanation: Since the first letters of s1 and s2 differ, they cannot be made equal.
