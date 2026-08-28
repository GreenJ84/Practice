// No-Zero integer is a positive integer that does not contain any 0 in its decimal representation.

// Given an integer n, return a list of two integers [a, b] where:

// a and b are No-Zero integers.
// a + b = n
// The test cases are generated so that there is at least one valid solution. If there are many valid solutions, you can return any of them.

// Constraints:
// 2 <= n <= 104

struct Solution;
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut small = 1;
        while small <= (n / 2) {
            if Self::check_int(small) && Self::check_int(n - small) {
                break;
            }
            small += 1;
        }
        vec![small, n - small]
    }

    fn check_int(mut n: i32) -> bool {
        while n > 0 {
            if n % 10 == 0 {
                return false;
            }
            n /= 10;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let expected = vec![1, 1];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }

    #[test]
    fn test_2() {
        let n = 11;
        let expected = vec![2, 9];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }

    #[test]
    fn test_3() {
        let n = 8018;
        let expected = vec![19, 7999];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }

    #[test]
    fn test_4() {
        let n = 18;
        let expected = vec![1, 17];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }
}
