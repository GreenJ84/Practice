// You are given an integer n.

// Return the concatenation of the hexadecimal representation of n2 and the hexatrigesimal representation of n3.

// A hexadecimal number is defined as a base-16 numeral system that uses the digits 0 – 9 and the uppercase letters A - F to represent values from 0 to 15.

// A hexatrigesimal number is defined as a base-36 numeral system that uses the digits 0 – 9 and the uppercase letters A - Z to represent values from 0 to 35.

// Constraints:
// 1 <= n <= 1000

struct Solution;
impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let hex = format!("{:X}", n * n);
        let hex36 = Solution::to_base36(n * n * n);
        format!("{hex}{hex36}")
    }

    fn to_base36(mut n: i32) -> String {
        let mut result = String::new();
        while n > 0 {
            let digit = (n % 36) as u8;
            let c = if digit < 10 {
                (digit + b'0') as char
            } else {
                (digit - 10 + b'A') as char
            };
            result.push(c);
            n /= 36;
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 13;
        let result = Solution::concat_hex36(n);
        assert_eq!(result, "A91P1");
    }

    #[test]
    fn test_2() {
        let n = 36;
        let result = Solution::concat_hex36(n);
        assert_eq!(result, "5101000");
    }
}
