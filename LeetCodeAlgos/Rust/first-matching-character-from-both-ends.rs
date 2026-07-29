// You are given a string s of length n consisting of lowercase English letters.

// Return the smallest index i such that s[i] == s[n - i - 1].

// If no such index exists, return -1.

// Constraints:
// 1 <= n == s.length <= 100
// s consists of lowercase English letters.

struct Solution;
impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i <= (n - i - 1) {
            if s[i] == s[n - i - 1] {
                return i as i32;
            }
            i += 1
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcacbd".to_string();
        let result = Solution::first_matching_index(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = "abc".to_string();
        let result = Solution::first_matching_index(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let s = "abcdab".to_string();
        let result = Solution::first_matching_index(s);
        assert_eq!(result, -1);
    }
}
