// Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.

// If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.

struct Solution;
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut v: Vec<char> = s.chars().collect();
        let n = v.len();
        let k = k as usize;

        let step = 2 * k;
        let mut i = 0usize;
        while i < n {
            let end = (i + k).min(n);
            v[i..end].reverse();
            i += step;
        }

        v.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example1() {
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd".to_string());
  }

  #[test]
  fn k_greater_than_length() {
    assert_eq!(Solution::reverse_str("abc".to_string(), 5), "cba".to_string());
  }

  #[test]
  fn k_equals_one() {
    assert_eq!(Solution::reverse_str("abcdef".to_string(), 1), "abcdef".to_string());
  }

  #[test]
  fn less_than_2k_but_ge_k() {
    assert_eq!(Solution::reverse_str("abcdefgh".to_string(), 5), "edcbafgh".to_string());
  }

  #[test]
  fn single_character() {
    assert_eq!(Solution::reverse_str("a".to_string(), 2), "a".to_string());
  }
}