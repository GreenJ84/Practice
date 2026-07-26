// Given an integer n, return a string with n characters such that each character in such string occurs an odd number of times.

// The returned string must contain only lowercase English letters. If there are multiples valid strings, return any of them.

// Constraints:
// 1 <= n <= 500

struct Solution;
impl Solution {
    pub fn generate_the_string1(n: i32) -> String {
        if n % 2 == 1 {
          "a".repeat(n as usize)
        } else {
          let mut ans = "a".repeat(n as usize - 1);
          ans.push('b');
          ans
        }
    }

    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 1 {
            std::iter::repeat_n('a', n as usize).collect()
        } else {
            std::iter::repeat_n('a', n as usize - 1).chain(std::iter::once('b')).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_string(s: &str) -> bool {
        let mut counts = std::collections::HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        for count in counts.values() {
            if count % 2 == 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_1() {
        let n = 4;
        let result = Solution::generate_the_string(n);
        assert_eq!(result.len(), 4);
        assert!(validate_string(&result));
    }

    #[test]
    fn test_2() {
        let n = 2;
        let result = Solution::generate_the_string(n);
        assert_eq!(result.len(), 2);
        assert!(validate_string(&result));
    }

    #[test]
    fn test_3() {
        let n = 7;
        let result = Solution::generate_the_string(n);
        assert_eq!(result.len(), 7);
        assert!(validate_string(&result));
    }
}
