// You are given an integer n and a digit x.

// A number is considered valid if:

// It contains at least one occurrence of digit x, and
// It does not start with digit x.
// Return true if n is valid, otherwise return false.

// Constraints:
// 0 <= n <= 10^5​​​​​​​
// 0 <= x <= 9

struct Solution;
impl Solution {
    pub fn valid_digit(mut n: i32, x: i32) -> bool {
        let mut ans = false;
        while n > 9 {
            if n % 10 == x {
                ans = true;
            }
            n /= 10;
        }
        ans && n != x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 101;
        let x = 0;
        assert_eq!(Solution::valid_digit(n, x), true);
    }

    #[test]
    fn test_2() {
        let n = 232;
        let x = 2;
        assert_eq!(Solution::valid_digit(n, x), false);
    }

    #[test]
    fn test_3() {
        let n = 5;
        let x = 1;
        assert_eq!(Solution::valid_digit(n, x), false);
    }
}
