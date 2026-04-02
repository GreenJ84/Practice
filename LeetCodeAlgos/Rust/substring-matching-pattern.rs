// You are given a string s and a pattern string p, where p contains exactly one '*' character.

// The '*' in p can be replaced with any sequence of zero or more characters.

// Return true if p can be made a substring of s, and false otherwise.

struct Solution;
impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let mut idx = 0usize;
        for part in p.split('*') {
            if let Some(part_start) = s[idx..].find(part) {
                idx += part_start + part.len();
            } else {
                return false;
            }
        }
        true
    }

    pub fn _has_match1(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let parts = p
            .split('*')
            .map(|p| p.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        let prefix = parts[0].to_owned();
        let suffix = parts[1].to_owned();
        if prefix.is_empty() && suffix.is_empty() {
            return true;
        } else if s.len() < prefix.len() + suffix.len() {
            return false;
        }


        let mut first_p = if prefix.is_empty() { Some(0) } else { None };
        for i in 0..s.len() {
            if first_p.is_none() && s[i] == prefix[0] {
                if i + prefix.len() <= s.len() && s[i..i + prefix.len()] == prefix {
                    if suffix.is_empty() {
                        return true;
                    }
                    first_p = Some(i + prefix.len());
                }
            }

            if !suffix.is_empty() && s[i] == suffix[0] {
                if i + suffix.len() < s.len() && s[i..i + suffix.len()] == suffix {
                    if first_p.is_some() && i >= first_p.unwrap() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let p = "ee*e".to_string();
        assert_eq!(Solution::has_match(s, p), true);
    }

    #[test]
    fn test_2() {
        let s = "car".to_string();
        let p = "c*v".to_string();
        assert_eq!(Solution::has_match(s, p), false);
    }
}

// Example 1:

// Input: s = "leetcode", p = "ee*e"

// Output: true

// Explanation:

// By replacing the '*' with "tcod", the substring "eetcode" matches the pattern.

// Example 2:

// Input: s = "car", p = "c*v"

// Output: false

// Explanation:

// There is no substring matching the pattern.

// Example 3:

// Input: s = "luck", p = "u*"

// Output: true

// Explanation:

// The substrings "u", "uc", and "uck" match the pattern.
