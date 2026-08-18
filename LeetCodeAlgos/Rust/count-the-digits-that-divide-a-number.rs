// Given an integer num, return the number of digits in num that divide num.

// An integer val divides nums if nums % val == 0.

// Constraints:
// 1 <= num <= 10^9
// num does not contain 0 as one of its digits.

struct Solution;
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut count = 0;
        let mut n = num;
        while n > 0 {
            if num % (n % 10) == 0 {
                count += 1;
            }
            n /= 10;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 7;
        let result = Solution::count_digits(num);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let num = 121;
        let result = Solution::count_digits(num);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let num = 1248;
        let result = Solution::count_digits(num);
        assert_eq!(result, 4);
    }
}
