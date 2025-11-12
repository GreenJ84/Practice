// Two strings word1 and word2 are considered almost equivalent if the differences between the frequencies of each letter from 'a' to 'z' between word1 and word2 is at most 3.

// Given two strings word1 and word2, each of length n, return true if word1 and word2 are almost equivalent, or false otherwise.

// The frequency of a letter x is the number of times it occurs in the string.

struct Solution;

use std::collections::HashMap;
impl Solution {
  pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut freq = HashMap::<char, i32>::new();
    for ch in word1.chars() {
      freq.entry(ch)
        .and_modify(|fr| *fr += 1)
        .or_insert(1);
    }

    for ch in word2.chars() {
      freq.entry(ch)
        .and_modify(|fr| *fr -= 1)
        .or_insert(-1);
    }

    for value in freq.into_values() {
      if value > 3 || value < -3 { return false; }
    }
    true
  }
}