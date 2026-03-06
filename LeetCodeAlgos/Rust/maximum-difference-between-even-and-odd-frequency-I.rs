// You are given a string s consisting of lowercase English letters.

// Your task is to find the maximum difference diff = freq(a1) - freq(a2) between the frequency of characters a1 and a2 in the string such that:

// a1 has an odd frequency in the string.
// a2 has an even frequency in the string.
// Return this maximum difference.

struct Solution;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [0; 26];
        for ch in s.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }

        let (mut max_odd, mut min_even) = (i32::MIN, i32::MAX);
        for &f in freq.iter() {
          if f % 2 == 1 {
              max_odd = max_odd.max(f);
          } else if f > 0 {
              min_even = min_even.min(f);
          }
        }
        max_odd - min_even
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let s = "aaaaabbc".to_string();
    assert_eq!(Solution::max_difference(s), 3);
  }

  #[test]
  fn test_2() {
    let s = "abcabcab".to_string();
    assert_eq!(Solution::max_difference(s), 1);
  }
}