// Given a string s and a character letter, return the percentage of characters in s that equal letter rounded down to the nearest whole percent.

struct Solution {}
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let n = s.len();
        let mut count: f64 = 0.0;
        for c in s.chars() {
            if c == letter {
                count += 1.0;
            }
        }
        (count / n as f64 * 100.0) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_letter() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
    }

    #[test]
    fn test_percentage_letter2() {
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}