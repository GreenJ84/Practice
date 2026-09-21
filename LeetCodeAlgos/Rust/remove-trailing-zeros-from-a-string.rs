// Given a positive integer num represented as a string, return the integer num without trailing zeros as a string.

// Constraints:
// 1 <= num.length <= 1000
// num consists of only digits.
// num doesn't have any leading zeros.

struct Solution;
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches('0').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = "51230100".to_string();
        assert_eq!(Solution::remove_trailing_zeros(num), "512301".to_string());
    }

    #[test]
    fn test_2() {
        let num = "123".to_string();
        assert_eq!(Solution::remove_trailing_zeros(num), "123".to_string());
    }
}
