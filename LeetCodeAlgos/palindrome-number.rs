// Given an integer x, return true if x is a palindrome, and false otherwise.

struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut start: usize = 0;
        let x = x.to_string().chars().collect::<Vec<char>>();
        let mut end: usize = x.len() - 1;
        while start <= end {
            if x[start] != x[end] { return false; }
            start += 1;
            if end != 0 { end -= 1; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_is_palindrome2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_is_palindrome3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_is_palindrome4() {
        assert_eq!(Solution::is_palindrome(0), true);
    }
}