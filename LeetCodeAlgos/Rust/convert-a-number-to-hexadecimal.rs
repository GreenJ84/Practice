// Given a 32-bit integer num, return a string representing its hexadecimal representation. For negative integers, two’s complement method is used.

// All the letters in the answer string should be lowercase characters, and there should not be any leading zeros in the answer except for the zero itself.

// Note: You are not allowed to use any built-in library method to directly solve this problem.

// Constraints:
// -231 <= num <= 231 - 1

struct Solution;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let neg = num < 0;
        let mut n = if neg { (num + 1).abs() } else { num };
        let mut ans = vec![];
        while n > 0 {
            ans.push((n % 16) as u8);
            n /= 16;
        }

        if neg {
            ans.extend(vec![0; 8 - ans.len()]);
        }
        ans.reverse();
        ans.into_iter()
            .map(|b| match (neg, b) {
                (true, b) => {
                    let dig = 15 - b;
                    if dig < 10 {
                        (b'0' + dig) as char
                    } else {
                        (b'a' + (dig - 10)) as char
                    }
                }
                (_, b) if b < 10 => (b'0' + b) as char,
                (_, b) => (b'a' + (b - 10)) as char,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 26;
        let expected = "1a".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_2() {
        let num = -1;
        let expected = "ffffffff".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_3() {
        let num = 161;
        let expected = "a1".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_4() {
        let num = 256;
        let expected = "100".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_5() {
        let num = 257;
        let expected = "101".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_6() {
        let num = i32::MAX;
        let expected = "7fffffff".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }

    #[test]
    fn test_7() {
        let num = i32::MIN;
        let expected = "80000000".to_string();
        assert_eq!(Solution::to_hex(num), expected);
    }
}
