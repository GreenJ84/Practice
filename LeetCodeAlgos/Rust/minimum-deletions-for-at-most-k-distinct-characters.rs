// You are given a string s consisting of lowercase English letters, and an integer k.

// Your task is to delete some (possibly none) of the characters in the string so that the number of distinct characters in the resulting string is at most k.

// Return the minimum number of deletions required to achieve this.

// Constraints:
// 1 <= s.length <= 16
// 1 <= k <= 16
// s consists only of lowercase English letters.

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let k = k as usize;
        if s.len() < k { return 0; }
        let mut freq = HashMap::<char, i32>::new();
        for ch in s.chars() {
          *freq.entry(ch).or_insert(0) += 1;
        }
        let mut freq: Vec<i32> = freq.into_values().collect();
        freq.sort_unstable();
        if freq.len() < k { return 0; }

        let mut ans = 0;
        for i in 0..(freq.len() - k) {
          ans += freq[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let s = "abc".to_string();
    let k = 2;
    assert_eq!(Solution::min_deletion(s, k), 1);
  }

  #[test]
  fn test_2() {
    let s = "aabb".to_string();
    let k = 2;
    assert_eq!(Solution::min_deletion(s, k), 0);
  }

  #[test]
  fn test_3() {
    let s = "yyyzz".to_string();
    let k = 1;
    assert_eq!(Solution::min_deletion(s, k), 2);
  }

  #[test]
  fn test_4() {
    let s = "uxyhf".to_string();
    let k = 7;
    assert_eq!(Solution::min_deletion(s, k), 0);
  }
}

// Example 3:

// Input: s = "yyyzz", k = 1

// Output: 2

// Explanation:

// s has two distinct characters ('y' and 'z') with frequencies of 3 and 2, respectively.
// Since we can have at most k = 1 distinct character, remove all occurrences of any one character from the string.
// Removing all 'z' results in at most k distinct characters. Thus, the answer is 2.
