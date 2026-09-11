// A perfect number is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.

// Given an integer n, return true if n is a perfect number, otherwise return false.

// Constraints:
// 1 <= num <= 10^8

struct Solution;
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut sum = 0;
        for i in 1..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                let divisor = num / i;
                if divisor != num {
                    if divisor != i {
                        sum += divisor;
                    }
                }
                sum += i;
            }
            if sum > num {
                return false;
            }
        }
        sum == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 28;
        assert_eq!(Solution::check_perfect_number(num), true);
    }

    #[test]
    fn test_2() {
        let num = 7;
        assert_eq!(Solution::check_perfect_number(num), false);
    }
}
