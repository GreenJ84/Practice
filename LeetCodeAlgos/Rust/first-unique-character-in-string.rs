// Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c| {
            map.insert(c, *map.get(&c).unwrap_or(&0) + 1);
        });
        for (idx, ch) in s.chars().enumerate() {
            if *map.get(&ch).unwrap_or(&0) == 1 {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char_1() {
        assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
    }

    #[test]
    fn test_first_uniq_char_2() {
        assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
    }

    #[test]
    fn test_first_uniq_char_3() {
        assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
    }
}
