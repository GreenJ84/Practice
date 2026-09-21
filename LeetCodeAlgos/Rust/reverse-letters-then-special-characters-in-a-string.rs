// You are given a string s consisting of lowercase English letters and special characters.

// Your task is to perform these in order:

// Reverse the lowercase letters and place them back into the positions originally occupied by letters.
// Reverse the special characters and place them back into the positions originally occupied by special characters.
// Return the resulting string after performing the reversals.

// Constraints:
// 1 <= s.length <= 100
// s consists only of lowercase English letters and the special characters in "!@#$%^&*()".

struct Solution;

use std::mem;
impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.iter_mut().filter(|c| c.is_ascii_lowercase());

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next_back() {
                mem::swap(left, right);
            } else {
                break;
            }
        }

        let mut iter = s.iter_mut().filter(|c| !c.is_ascii_lowercase());

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next_back() {
                mem::swap(left, right);
            } else {
                break;
            }
        }

        String::from_utf8(s).unwrap()
    }

    pub fn reverse_by_type1(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0usize;
        let mut right = chars.len() - 1;
        let mut tmp = ' ';
        while left < right {
            while !chars[left].is_ascii_lowercase() && left < right {
                left += 1;
            }
            while !chars[right].is_ascii_lowercase() && right > left {
                right -= 1;
            }
            if left >= right {
                break;
            }
            tmp = chars[right];
            chars[right] = chars[left];
            chars[left] = tmp;
            left += 1;
            right -= 1;
        }
        left = 0;
        right = chars.len() - 1;
        while left < right {
            while chars[left].is_ascii_lowercase() {
                left += 1;
            }
            while chars[right].is_ascii_lowercase() && right > left {
                right -= 1;
            }
            if left >= right {
                break;
            }
            tmp = chars[right];
            chars[right] = chars[left];
            chars[left] = tmp;
            left += 1;
            right -= 1;
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = ")ebc#da@f(".to_string();
        let expected = "(fad@cb#e)".to_string();
        assert_eq!(Solution::reverse_by_type(s), expected);
    }

    #[test]
    fn test_2() {
        let s = "z".to_string();
        let expected = "z".to_string();
        assert_eq!(Solution::reverse_by_type(s), expected);
    }

    #[test]
    fn test_3() {
        let s = "!@#$%^&*()".to_string();
        let expected = ")(*&^%$#@!".to_string();
        assert_eq!(Solution::reverse_by_type(s), expected);
    }
}
