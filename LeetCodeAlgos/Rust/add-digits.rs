// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

struct Solution {}
impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num >= 10 {
            let mut curr = num;
            let mut sum = 0;
            while curr >= 10 {
                sum += curr % 10;
                curr /= 10;
            }
            num = sum + curr;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_digits() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn test_add_digits2() {
        assert_eq!(Solution::add_digits(0), 0);
    }
    #[test]
    fn test_add_digits3() {
        assert_eq!(Solution::add_digits(101), 2);
    }
}