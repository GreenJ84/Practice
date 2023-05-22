// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
// Given a string s, return true if it is a palindrome, or false otherwise.

struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let ss: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap().to_ascii_lowercase())
            .collect();

        if ss.len() < 2{
            return true;
        }
        let mut start = 0;
        let mut end = ss.len() - 1;
        while start <= end {
            if ss[start] != ss[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }

    #[test]
    fn test_is_palindrome2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_is_palindrome3() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}

