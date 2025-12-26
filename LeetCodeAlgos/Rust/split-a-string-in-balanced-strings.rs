// Balanced strings are those that have an equal quantity of 'L' and 'R' characters.

// Given a balanced string s, split it into some number of substrings such that:

// Each substring is balanced.
// Return the maximum number of balanced strings you can obtain.

struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
      let (mut balance, mut count) = (0i32, 0i32);
      for ch in s.chars() {
        match ch {
          'L' => { balance += 1; },
          _ => { balance -= 1; }
        }
        if balance == 0 {
          count += 1;
        }
      }
      count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::balanced_string_split("RL".to_string()), 1);
        assert_eq!(Solution::balanced_string_split("LR".to_string()), 1);
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(Solution::balanced_string_split("RLRLRLRL".to_string()), 4);
        assert_eq!(Solution::balanced_string_split("LRLRLRLR".to_string()), 4);
    }

    #[test]
    fn test_nested_balance() {
        assert_eq!(Solution::balanced_string_split("RLLR".to_string()), 2);
        assert_eq!(Solution::balanced_string_split("LRRL".to_string()), 2);
    }

    #[test]
    fn test_mixed_lengths() {
        assert_eq!(Solution::balanced_string_split("RRLLRRLL".to_string()), 2);
        assert_eq!(Solution::balanced_string_split("RLLLRRRRLR".to_string()), 2);
    }

    #[test]
    fn test_long_balanced() {
        assert_eq!(
            Solution::balanced_string_split("LLLLLLRRRRRR".to_string()),
            1
        );
        assert_eq!(
            Solution::balanced_string_split("RRRRRRLLLLLLL".to_string()),
            1
        );
    }
}
