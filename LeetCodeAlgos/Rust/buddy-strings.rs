// Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.

// Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].

// For example, swapping at indices 0 and 2 in "abcd" results in "cbad".

// Constraints:
// 1 <= s.length, goal.length <= 2 * 104
// s and goal consist of lowercase letters.

struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut diff = 0u8;
        let mut pair = false;
        let mut check = vec![[0u8; 2]; 26];
        for (sc, gc) in s.bytes().zip(goal.bytes()) {
            if sc != gc {
                if diff == 2 {
                    return false;
                }
                diff += 1;
            }
            let sc = (sc - b'a') as usize;
            check[sc][0] += 1;
            check[(gc - b'a') as usize][1] += 1;
            if !pair && check[sc][0] > 1 {
                pair = true;
            }
        }
        !check.iter().any(|x| x[0] != x[1]) && diff == 2 || (diff == 0 && pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ab".to_string();
        let goal = "ba".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = "ab".to_string();
        let goal = "ab".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s = "aa".to_string();
        let goal = "aa".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, true);
    }

    #[test]
    fn test_4() {
        let s = "ab".to_string();
        let goal = "xw".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, false);
    }

    #[test]
    fn test_5() {
        let s = "ab".to_string();
        let goal = "babbb".to_string();
        let result = Solution::buddy_strings(s, goal);
        assert_eq!(result, false);
    }
}
