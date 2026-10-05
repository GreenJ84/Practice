// You are given a string s and two distinct lowercase English letters x and y.

// Rearrange the characters of s to construct a new string t such that:

// t is a permutation of s.
// Every occurrence of y appears before every occurrence of x in t.
// Return any valid string t.

// Constraints:
// 1 <= s.length <= 100
// s consists of lowercase English letters.
// x and y are lowercase English letters.
// x != y

struct Solution;
impl Solution {
    pub fn rearrange_string(s: String, x: char, y: char) -> String {
        let mut ans = String::new();
        let mut end = 0;
        for ch in s.chars() {
            if ch == x {
                end += 1;
                continue;
            }
            ans.push(ch);
        }
        if end == 0 {
            return ans;
        }
        ans.extend(std::iter::repeat_n(x, end));
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aabc".to_string();
        let x = 'a';
        let y = 'c';
        let result = Solution::rearrange_string(s, x, y);
        assert_eq!(result, "bcaa");
    }

    #[test]
    fn test_2() {
        let s = "dcab".to_string();
        let x = 'd';
        let y = 'b';
        let result = Solution::rearrange_string(s, x, y);
        assert_eq!(result, "cabd");
    }

    #[test]
    fn test_3() {
        let s = "axe".to_string();
        let x = 'o';
        let y = 'x';
        let result = Solution::rearrange_string(s, x, y);
        assert_eq!(result, "axe");
    }
}
