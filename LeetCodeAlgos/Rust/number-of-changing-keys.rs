// You are given a 0-indexed string s typed by a user. Changing a key is defined as using a key different from the last used key. For example, s = "ab" has a change of a key while s = "bBBb" does not have any.

// Return the number of times the user had to change the key.

// Note: Modifiers like shift or caps lock won't be counted in changing the key that is if a user typed the letter 'a' and then the letter 'A' then it will not be considered as a changing of key.

struct Solution {}

use std::char::ToLowercase;
impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let key_chars: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut last_seen: ToLowercase = key_chars.first().unwrap().to_lowercase();
        let mut count: i32 = 0;
        for i in 1..s.len() {
            let current = key_chars.get(i).unwrap().to_lowercase();
            print!("current: {}, last_seen: {}, equals: {} \n", current, last_seen, current.clone().eq(last_seen.clone()));
            if !current.clone().eq(last_seen.clone()) {
                count += 1;
                last_seen = current.clone();
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_key_changes1() {
        let result = Solution::count_key_changes("aAbBcC".to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_key_changes_2() {
        let result = Solution::count_key_changes("AaAaAaaA".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_key_changes_3() {
        let result = Solution::count_key_changes("mDVD".to_string());
        assert_eq!(result, 3);
    }
}