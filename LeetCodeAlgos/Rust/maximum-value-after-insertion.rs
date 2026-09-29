// You are given a very large integer n, represented as a string,​​​​​​ and an integer digit x. The digits in n and the digit x are in the inclusive range [1, 9], and n may represent a negative number.

// You want to maximize n's numerical value by inserting x anywhere in the decimal representation of n​​​​​​. You cannot insert x to the left of the negative sign.

// For example, if n = 73 and x = 6, it would be best to insert it between 7 and 3, making n = 763.
// If n = -55 and x = 2, it would be best to insert it before the first 5, making n = -255.
// Return a string representing the maximum value of n​​​​​​ after the insertion.

// Constraints:
// 1 <= n.length <= 10^5
// 1 <= x <= 9
// The digits in n​​​ are in the range [1, 9].
// n is a valid representation of an integer.
// In the case of a negative n,​​​​​​ it will begin with '-'.

struct Solution;
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut bytes: Vec<u8> = n.into_bytes();
        let n = bytes.len();
        let x = b'0' + x as u8;
        if bytes[0] == '-' as u8 {
            for i in 1..n {
                if bytes[i] > x {
                    bytes.insert(i, x);
                    return String::from_utf8(bytes).unwrap();
                }
                bytes.push(x);
            }
        } else {
            for i in 0..n {
                if bytes[i] < x {
                    bytes.insert(i, x);
                    return String::from_utf8(bytes).unwrap();
                }
            }
            bytes.push(x);
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = "99".to_string();
        let x = 9;
        let result = Solution::max_value(n, x);
        assert_eq!(result, "999");
    }

    #[test]
    fn test_2() {
        let n = "-13".to_string();
        let x = 2;
        let result = Solution::max_value(n, x);
        assert_eq!(result, "-123");
    }
}
