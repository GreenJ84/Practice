// Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.


struct Solution{}
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut x = s.chars().collect::<Vec<char>>();
        for _ in 0..(x.len() / 2 as usize) {
            let first = x[0].clone();
            x = x[1..(s.len() as usize)].to_vec();
            x.push(first);

            if x.clone().into_iter().collect::<String>() == s {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_repeated_substring_pattern() {
        assert_eq!(Solution::repeated_substring_pattern("abab".to_string()), true);
    }

    #[test]
    fn test_repeated_substring_pattern2() {
        assert_eq!(Solution::repeated_substring_pattern("aba".to_string()), false);
    }

    #[test]
    fn test_repeated_substring_pattern3() {
        assert_eq!(Solution::repeated_substring_pattern("abcabcabcabc".to_string()), true);
    }

    
    #[test]
    fn test_repeated_substring_pattern4() {
        assert_eq!(Solution::repeated_substring_pattern("bbbbb".to_string()), true);
    }
    
    #[test]
    fn test_repeated_substring_pattern5() {
        assert_eq!(Solution::repeated_substring_pattern("pwwkew".to_string()), false);
    }
    
    #[test]
    fn test_repeated_substring_pattern6() {
        assert_eq!(Solution::repeated_substring_pattern("abcdefghijklmnopqrstuvwxyz".to_string()), false);
    }

    #[test]
    fn test_repeated_substring_pattern7() {
        assert_eq!(Solution::repeated_substring_pattern("a".to_string()), false);
    }
}