// Given a string s, return true if s is a good string, or false otherwise.
// A string s is good if all the characters that appear in s have the same number of occurrences (i.e., the same frequency).

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut basis: (char, u32) = ('a', 0);
        let mut map: HashMap<char, u32> = HashMap::new();
        for (idx, ch) in s.chars().enumerate() {
            if idx == 0 { basis.0 = ch.clone(); }
            *map.entry(ch).or_insert(0) += 1;
            if basis.0 == ch { basis.1 += 1; }
        }
        for (_, val) in map.into_values().enumerate() {
            if val != basis.1 { return false; }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_occurrences_equal() {
        assert_eq!(Solution::are_occurrences_equal(String::from("abacbc")), true);
    }

    #[test]
    fn test_are_occurrences_equal_2() {
        assert_eq!(Solution::are_occurrences_equal(String::from("aaabb")), false);
    }
}