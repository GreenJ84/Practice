// You are given a string s and an integer k.

// Determine if there exists a substring of length exactly k in s that satisfies the following conditions:

// The substring consists of only one distinct character (e.g., "aaa" or "bbb").
// If there is a character immediately before the substring, it must be different from the character in the substring.
// If there is a character immediately after the substring, it must also be different from the character in the substring.
// Return true if such a substring exists. Otherwise, return false.

struct Solution;
impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let (mut left, mut right) = (0usize, 0usize);
        let mut last = ' ';
        for ch in s.chars() {
          println!("l:{}-r:{}", left, right);
          println!("curr: ({})", &last);
          if ch != last {
            if right - left + 1 == k as usize {
              return true;
            }
            left = right + 1;
            last = ch;
            println!("reset - new last: {}", &ch);
          }
          right += 1;
        }
        return right - left == k as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "aaabaaa".to_string();
        let k = 3;
        assert_eq!(Solution::has_special_substring(s, k), true);
    }

    #[test]
    fn test2() {
        let s = "abc".to_string();
        let k = 2;
        assert_eq!(Solution::has_special_substring(s, k), false);
    }
}