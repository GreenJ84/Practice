// Given a string s consisting of only the characters 'a' and 'b', return true if every 'a' appears before every 'b' in the string. Otherwise, return false.

struct Solution;
impl Solution {
  pub fn check_string(s: String) -> bool {
      let mut b = false;
        for ch in s.chars() {
          if ch == 'b' && !b {
            b = true;
          }
          else if ch == 'a' && b {
            return false;
          }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aaabbb".to_string();
        assert_eq!(Solution::check_string(s), true);
    }

    #[test]
    fn test_2() {
        let s = "abab".to_string();
        assert_eq!(Solution::check_string(s), false);
    }

    #[test]
    fn test_3() {
        let s = "bbb".to_string();
        assert_eq!(Solution::check_string(s), true);
    }
}
