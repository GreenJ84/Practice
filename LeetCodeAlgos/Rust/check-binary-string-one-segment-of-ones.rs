// Given a binary string s ​​​​​without leading zeros, return true​​​ if s contains at most one contiguous segment of ones. Otherwise, return false.

struct Solution {}
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let mut sequence_end = false;

        for idx in 1..s.len() {
            if s[idx] == '0' {
                sequence_end = true;
            } else if s[idx] == '1' {
                if sequence_end {
                    return false;
                }
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ones_segment() {
        assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
    }
    
    #[test]
    fn test_check_ones_segment2() {
        assert_eq!(Solution::check_ones_segment("110".to_string()), true);
    }

    #[test]
    fn test_check_ones_segment3() {
        assert_eq!(Solution::check_ones_segment("111011".to_string()), false);
    }
    #[test]
    fn test_check_ones_segment4() {
        assert_eq!(Solution::check_ones_segment("101101".to_string()), false);
    }
    #[test]
    fn test_check_ones_segment5() {
        assert_eq!(Solution::check_ones_segment("10101".to_string()), false);
    }

    #[test]
    fn test_check_ones_segment6() {
        assert_eq!(Solution::check_ones_segment("10100".to_string()), false);
    }

    #[test]
    fn test_check_ones_segment7() {
        assert_eq!(Solution::check_ones_segment("1".to_string()), false);
    }
}