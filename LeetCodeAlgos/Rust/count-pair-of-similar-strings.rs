// You are given a 0-indexed string array words.

// Two strings are similar if they consist of the same characters.

// For example, "abca" and "cba" are similar since both consist of characters 'a', 'b', and 'c'.
// However, "abacba" and "bcfd" are not similar since they do not consist of the same characters.
// Return the number of pairs (i, j) such that 0 <= i < j <= word.length - 1 and the two strings words[i] and words[j] are similar.

struct Solution;

use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
  pub fn similar_pairs(words: Vec<String>) -> i32 {
    const A: u32 = 'a' as u32;

    let mut counts: HashMap<u32, i32> = HashMap::new();
    let mut ans = 0;

    for word in words {
      let mut mask = 0u32;
      word.chars().for_each(|ch| mask |= 1 << ch as u32 - A);
      let count = counts.entry(mask).or_insert(0);
      ans += * count;
      *count += 1;
    }
    ans
  }

  pub fn similar_pairs1(words: Vec<String>) -> i32 {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    {
      let mut set: HashSet<char>;
      for word in words {
        set = HashSet::<char>::new();
        for char in word.chars() {
          set.insert(char);
        }
        sets.push(set);
      }
    }

    let mut ans = 0;
    for i in 0..sets.len() - 1 {
      for j in i + 1..sets.len() {
        if sets[i].symmetric_difference(&sets[j]).count() == 0 {
          ans += 1;
        }
      }
    }
    ans
  }
}