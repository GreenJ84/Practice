// The power of the string is the maximum length of a non-empty substring that contains only one unique character.

// Given a string s, return the power of s.

struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
      if s.is_empty() { return 0; }
      let mut ans = 1i32;

      let mut s = s.chars();
      let mut last = s.next().unwrap();

      let mut run = 1i32;
      for ch in s {
        if ch == last {
          run += 1
        } else {
          ans = ans.max(run);
          run = 1;
          last = ch;
        }
      }
      ans.max(run)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_power("leetcode".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::max_power("a".to_string()), 1);
    }

    #[test]
    fn all_same_characters() {
        assert_eq!(Solution::max_power("aaaa".to_string()), 4);
    }

    #[test]
    fn alternating_characters() {
        assert_eq!(Solution::max_power("abab".to_string()), 1);
    }
}
