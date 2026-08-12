// You are given a positive integer n.

// Let digitSum be the sum of the digits of n, and let squareSum be the sum of the squares of the digits of n.

// An integer is called good if squareSum - digitSum >= 50.

// Return true if n is good. Otherwise, return false.

// Constraints:
// 1 <= n <= 109

struct Solution;
impl Solution {
    pub fn check_good_integer(mut n: i32) -> bool {
        let mut running_sum = 0;
        while n > 0 {
            let digit = n % 10;
            n /= 10;
            running_sum += digit * digit - digit;
        }
        running_sum >= 50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 1000;
        let expected = false;
        assert_eq!(Solution::check_good_integer(n), expected);
    }

    #[test]
    fn test_2() {
        let n = 19;
        let expected = true;
        assert_eq!(Solution::check_good_integer(n), expected);
    }
}
