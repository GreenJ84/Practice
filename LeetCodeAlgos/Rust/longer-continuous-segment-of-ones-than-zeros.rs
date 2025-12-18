// Given a binary string s, return true if the longest contiguous segment of 1's is strictly longer than the longest contiguous segment of 0's in s, or return false otherwise.

// For example, in s = "110100010" the longest continuous segment of 1s has length 2, and the longest continuous segment of 0s has length 3.
// Note that if there are no 0's, then the longest continuous segment of 0's is considered to have a length 0. The same applies if there is no 1's.

struct Solution;
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
      let s = s.chars().collect::<Vec<char>>();

      let mut contiguous_ones = 0;
      let mut contiguous_zeros = 0;
      let mut run = 1;

      for idx in 1..s.len(){
        if s[idx] == s[idx - 1] {
          run += 1;
          continue;
        }
        match s[idx] {
          '1' if run > contiguous_zeros => {
            contiguous_zeros = run;
          }
          '0' if run > contiguous_ones => {
            contiguous_ones = run;
          }
          _ => {}
        }
        run = 1;
      }
      match s[s.len() - 1] {
        '1' if run > contiguous_ones => {
          contiguous_ones = run;
        }
        '0' if run > contiguous_zeros => {
          contiguous_zeros = run;
        }
        _ => {}
      }

      contiguous_ones > contiguous_zeros
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
  }

  #[test]
  fn example_3() {
    assert_eq!(Solution::check_zero_ones("110100010".to_string()), false);
  }

  #[test]
  fn single_one() {
    assert_eq!(Solution::check_zero_ones("1".to_string()), true);
  }

  #[test]
  fn single_zero() {
    assert_eq!(Solution::check_zero_ones("0".to_string()), false);
  }

  #[test]
  fn all_ones() {
    assert_eq!(Solution::check_zero_ones("11111".to_string()), true);
  }

  #[test]
  fn all_zeros() {
    assert_eq!(Solution::check_zero_ones("00000".to_string()), false);
  }

  #[test]
  fn ones_longer() {
    assert_eq!(Solution::check_zero_ones("11100".to_string()), true);
  }

  #[test]
  fn alternating() {
    assert_eq!(Solution::check_zero_ones("101010".to_string()), false);
  }

  #[test]
  fn ones_at_end() {
    assert_eq!(Solution::check_zero_ones("0000111111".to_string()), true);
  }
}