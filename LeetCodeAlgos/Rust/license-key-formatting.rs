// You are given a license key represented as a string s that consists of only alphanumeric characters and dashes. The string is separated into n + 1 groups by n dashes. You are also given an integer k.

// We want to reformat the string s such that each group contains exactly k characters, except for the first group, which could be shorter than k but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.

// Return the reformatted license key.

struct Solution;
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut ans = String::new();
        let mut curr_group = 0;

        for character in s.chars().rev() {
          match character {
            ch if character.is_ascii_digit() || character.is_ascii_uppercase() => {
              ans.push(ch);
              curr_group += 1;
            },
            ch if character.is_ascii_lowercase() => {
              ans.push_str(&ch.to_uppercase().to_string());
              curr_group += 1;
            },
            _ => { continue; }
          }

          if curr_group == k {
            ans.push('-');
            curr_group = 0;
          }

        }
        ans = ans.chars().rev().collect();
        if ans.starts_with('-') {
          ans.remove(0);
        }
        ans
    }
}