// You are given a string s consisting of lowercase English letters.

// The frequency group for a value k is the set of characters that appear exactly k times in s.

// The majority frequency group is the frequency group that contains the largest number of distinct characters.

// Return a string containing all characters in the majority frequency group, in any order. If two or more frequency groups tie for that largest size, pick the group whose frequency k is larger.

struct Solution;

use std::collections::HashMap;
impl Solution {
  pub fn majority_frequency_group(s: String) -> String {
    let mut frequencies = HashMap::<char, i32>::new();
    for ch in s.chars() {
      frequencies.entry(ch)
      .and_modify(|x| *x += 1)
      .or_insert(1);
    }

    let mut groups = HashMap::<i32, Vec<char>>::new();
    for (ch, freq) in frequencies.into_iter() {
      groups.entry(freq)
        .and_modify(|x| x.push(ch))
        .or_insert(vec![ch]);
    }

    let (_, max_group) = groups.into_iter()
      .max_by(|(aFrq, aChars), (bFrq, bChars)| (aChars.len(), aFrq).cmp(&(bChars.len(), bFrq)))
      .unwrap();

    max_group.into_iter().collect()
  }
}