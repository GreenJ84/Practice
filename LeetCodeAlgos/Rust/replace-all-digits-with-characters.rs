// You are given a 0-indexed string s that has lowercase English letters in its even indices and digits in its odd indices.

// You must perform an operation shift(c, x), where c is a character and x is a digit, that returns the xth character after c.

// For example, shift('a', 5) = 'f' and shift('x', 0) = 'x'.
// For every odd index i, you want to replace the digit s[i] with the result of the shift(s[i-1], s[i]) operation.

// Return s after replacing all digits. It is guaranteed that shift(s[i-1], s[i]) will never exceed 'z'.

// Note that shift(c, x) is not a preloaded function, but an operation to be implemented as part of the solution.

struct Solution;
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();
        for i in (0..(s.len() / 2)).map(|i| i * 2 + 1) {
            s[i] = s[i - 1] + s[i] - 48;
        }
        s.into_iter().map(|b| b as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "a1c1e1".to_string();
        let expected = "abcdef".to_string();
        assert_eq!(Solution::replace_digits(s), expected);
    }

    #[test]
    fn test_2() {
        let s = "a1b2c3d4e".to_string();
        let expected = "abbdcfdhe".to_string();
        assert_eq!(Solution::replace_digits(s), expected);
    }
}
