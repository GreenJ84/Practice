// Given a string s, return the number of palindromic substrings in it.
// A string is a palindrome when it reads the same backward as forward.
// A substring is a contiguous sequence of characters within the string.

struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        // Easy outs before memory usage
        if s.is_empty() {
            return 0;
        } else if s.len() == 1 {
            return 1;
        }

        let mut substrings_count = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let single_center = Solution::expand_center(&chars, i as i32, i);
            let double_center = Solution::expand_center(&chars, i as i32, i + 1);
            substrings_count += single_center + double_center;
        }
        substrings_count
    }

    fn expand_center(s: &Vec<char>, mut left: i32, mut right: usize) -> i32 {
        let mut count = 0;
        while left >= 0 && right < s.len() && s[left as usize] == s[right] {
            count += 1;
            left -= 1;
            right += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings1() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn test_count_substrings2() {
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }
}
