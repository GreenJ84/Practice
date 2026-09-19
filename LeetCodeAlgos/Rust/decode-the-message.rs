// You are given the strings key and message, which represent a cipher key and a secret message, respectively. The steps to decode message are as follows:

// Use the first appearance of all 26 lowercase English letters in key as the order of the substitution table.
// Align the substitution table with the regular English alphabet.
// Each letter in message is then substituted using the table.
// Spaces ' ' are transformed to themselves.
// For example, given key = "happy boy" (actual key would have at least one instance of each letter in the alphabet), we have the partial substitution table of ('h' -> 'a', 'a' -> 'b', 'p' -> 'c', 'y' -> 'd', 'b' -> 'e', 'o' -> 'f').
// Return the decoded message.

// Constraints:
// 26 <= key.length <= 2000
// key consists of lowercase English letters and ' '.
// key contains every letter in the English alphabet ('a' to 'z') at least once.
// 1 <= message.length <= 2000
// message consists of lowercase English letters and ' '.

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut codex = HashMap::<u8, char>::with_capacity(26);
        {
            for &ch in key.as_bytes() {
                if !codex.contains_key(&ch) && ch != b' ' {
                    codex.insert(ch, (b'a' + codex.len() as u8) as char);
                }
            }
        }
        message
            .as_bytes()
            .iter()
            .map(|ch| codex.get(&ch).unwrap_or(&' '))
            .collect()
    }

    pub fn decode_message1(key: String, message: String) -> String {
        let mut codex = HashMap::<char, char>::with_capacity(26);
        {
            let mut diff = 0u8;
            for ch in key.chars() {
                if !codex.contains_key(&ch) && ch != ' ' {
                    codex.insert(ch, (b'a' + diff) as char);
                    diff += 1;
                }
            }
        }
        println!("{codex:?}");
        let mut ans = String::new();
        for ch in message.chars() {
            ans.push(match ch {
                ' ' => ' ',
                _ => *codex.get(&ch).unwrap(),
            });
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let expected = "this is a secret".to_string();
        assert_eq!(Solution::decode_message(key, message), expected);
    }

    #[test]
    fn test_2() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        let expected = "the five boxing wizards jump quickly".to_string();
        assert_eq!(Solution::decode_message(key, message), expected);
    }
}
