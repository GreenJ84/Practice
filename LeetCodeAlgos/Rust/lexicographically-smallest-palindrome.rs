// You are given a string s consisting of lowercase English letters, and you are allowed to perform operations on it. In one operation, you can replace a character in s with another lowercase English letter.

// Your task is to make s a palindrome with the minimum number of operations possible. If there are multiple palindromes that can be made using the minimum number of operations, make the lexicographically smallest one.

// A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b.

// Return the resulting palindrome string.

// Constraints:
// 1 <= s.length <= 1000
// s consists of only lowercase English letters.

struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let n = s.len();
        let mut chars = s.chars().collect::<Vec<char>>();
        for i in 0..(n / 2) {
            match chars[i].cmp(&chars[n - i - 1]){
                Ordering::Less => {
                    chars[n - i - 1] = chars[i];
                },
                Ordering::Greater => {
                    chars[i] = chars[n - i - 1];
                },
                Ordering::Equal => {},
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let s = "egcfe".to_string();
        let result = Solution::make_smallest_palindrome(s);
        assert_eq!(result, "efcfe");
    }

    #[test]
    fn test_2() {
        let s = "abcd".to_string();
        let result = Solution::make_smallest_palindrome(s);
        assert_eq!(result, "abba");
    }

    #[test]
    fn test_3() {
        let s = "seven".to_string();
        let result = Solution::make_smallest_palindrome(s);
        assert_eq!(result, "neven");
    }
}
