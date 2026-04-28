// Given two strings s and t, return 1 if t is an anagram of s, otherwise return 0.


// fn isAnagram(s: &str, t: &str) -> i32 {
fn is_anagram(s: &str, t: &str) -> i32 {
    let mut freq = [0; 26];
    for ch in s.chars() {
      freq[(ch as u8 - b'a') as usize] += 1;
    }

    for ch in t.chars() {
      freq[(ch as u8 - b'a') as usize] -= 1;
    }
    if freq.iter().all(|&x| x == 0) {
        1
    } else {
        0
    }
}

use std::collections::{HashMap, hash_map::Entry};
fn _is_anagram1(s: &str, t: &str) -> i32 {
    let mut freq = HashMap::<char, i32>::new();
    for ch in s.chars() {
      freq.entry(ch)
        .and_modify(|f| { *f += 1; })
        .or_insert(1);
    }

    for ch in t.chars() {
      if let Entry::Occupied(mut entry) = freq.entry(ch) {
        if *entry.get() == 1 {
          entry.remove();
        } else {
          *entry.get_mut() -= 1;
        }
      } else {
        return 0;
      }
    }
    if freq.is_empty() {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
      let s = "listen";
      let t = "silent";
      assert_eq!(is_anagram(s, t), 1);
    }

    #[test]
    fn test_2() {
      let s = "hello";
      let t = "bellow";
      assert_eq!(is_anagram(s, t), 0);
    }

    #[test]
    fn test_3() {
      let s = "anagram";
      let t = "anagrams";
      assert_eq!(is_anagram(s, t), 0);
    }
}

// Example 1

// Input:

// s = listen
// t = silent
// Output:

// 1
// Explanation:

// Both strings have length 6.
// Build a frequency map for 'listen': {l:1, i:1, s:1, t:1, e:1, n:1}.
// Iterate over 'silent': s→count 1→0, i→1→0, l→1→0, e→1→0, n→1→0, t→1→0.
// All counts return to zero and no mismatches occur, so they are anagrams.
// Example 2

// Input:

// s = hello
// t = bellow
// Output:

// 0
// Explanation:

// The strings have different lengths (5 vs 6). Since lengths differ, we can immediately conclude they are not anagrams and return 0.

// Input Format

// The input consists of exactly two lines:

// Line 1: A string s of length n, where 0 ≤ n ≤ 1000, containing only lowercase letters 'a' to 'z'.
// Line 2: A string t of length m, where 0 ≤ m ≤ 1000, containing only lowercase letters 'a' to 'z'.
// Both strings may be empty (n = 0 or m = 0).

// No extra spaces or characters appear on each line.