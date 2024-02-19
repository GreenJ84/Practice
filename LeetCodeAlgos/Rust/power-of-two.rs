// Given an integer n, return true if it is a power of two. Otherwise, return false.
// An integer n is a power of two, if there exists an integer x such that n == 2x.

struct Solution {}
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n == 0 { return false; }
        while n % 2 == 0 {
            n /= 2;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_power_of_two(16), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}