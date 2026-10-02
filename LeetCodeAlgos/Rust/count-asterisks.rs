// You are given a string s, where every two consecutive vertical bars '|' are grouped into a pair. In other words, the 1st and 2nd '|' make a pair, the 3rd and 4th '|' make a pair, and so forth.

// Return the number of '*' in s, excluding the '*' between each pair of '|'.

// Note that each '|' will belong to exactly one pair.

// Constraints:
// 1 <= s.length <= 1000
// s consists of lowercase English letters, vertical bars '|', and asterisks '*'.
// s contains an even number of vertical bars '|'.

struct Solution;
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut ans = 0;
        let mut between_bars = false;

        for &byte in s.as_bytes() {
            if byte == b'|' {
                between_bars = !between_bars;
            } else if between_bars {
                continue;
            } else if byte == b'*' {
                ans += 1;
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
        let s = "l|*e*et|c**o|*de|".to_string();
        assert_eq!(Solution::count_asterisks(s), 2);
    }

    #[test]
    fn test_2() {
        let s = "iamprogrammer".to_string();
        assert_eq!(Solution::count_asterisks(s), 0);
    }

    #[test]
    fn test_3() {
        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();
        assert_eq!(Solution::count_asterisks(s), 5);
    }
}
