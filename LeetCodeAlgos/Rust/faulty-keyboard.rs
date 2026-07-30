// Your laptop keyboard is faulty, and whenever you type a character 'i' on it, it reverses the string that you have written. Typing other characters works as expected.

// You are given a 0-indexed string s, and you type each character of s using your faulty keyboard.

// Return the final string that will be present on your laptop screen.

// Constraints:
// 1 <= s.length <= 100
// s consists of lowercase English letters.
// s[0] != 'i'

struct Solution;
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut ans = String::new();
        let s: Vec<char> = s.chars().collect();
        let mut flip = false;
        for idx in 0..s.len() {
            if s[idx] == 'i' {
                flip = !flip;
            } else {
                if flip {
                    ans = ans.chars().rev().collect();
                    flip = !flip;
                }
                ans.push(s[idx]);
            }
        }
        if flip {
            ans.chars().rev().collect()
        } else {
            ans
        }
    }

    pub fn final_string1(s: String) -> String {
        let mut ans = String::new();
        for ch in s.chars() {
            if ch == 'i' {
                ans = ans.chars().rev().collect();
            } else {
                ans.push(ch);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "string".to_string();
        let result = Solution::final_string(s);
        assert_eq!(result, "rtsng");
    }

    #[test]
    fn test_2() {
        let s = "poiinter".to_string();
        let result = Solution::final_string(s);
        assert_eq!(result, "ponter");
    }
}
