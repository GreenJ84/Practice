// You are given an integer n. Your task is to compute the GCD (greatest common divisor) of two values:

// sumOdd: the sum of the first n odd numbers.

// sumEven: the sum of the first n even numbers.

// Return the GCD of sumOdd and sumEven.

struct Solution;
impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let (mut odd, mut even) = (0, 0);
        for num in 1..=(2 * n) {
            if num % 2 == 0 {
                even += num as i32;
            } else {
                odd += num as i32;
            }
        }
        let (mut big, mut small) = if odd >= even {
            (odd, even)
        } else {
            (even, odd)
        };
        while small != 0 {
            (big, small) = (small, big % small);
        }
        big
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::gcd_of_odd_even_sums(4), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::gcd_of_odd_even_sums(5), 5);
    }

    #[test]
    fn test_n_1() {
        assert_eq!(Solution::gcd_of_odd_even_sums(1), 1);
    }

    #[test]
    fn test_n_2() {
        assert_eq!(Solution::gcd_of_odd_even_sums(2), 2);
    }

    #[test]
    fn test_n_3() {
        assert_eq!(Solution::gcd_of_odd_even_sums(3), 3);
    }

    #[test]
    fn test_n_6() {
        assert_eq!(Solution::gcd_of_odd_even_sums(6), 6);
    }

    #[test]
    fn test_n_10() {
        assert_eq!(Solution::gcd_of_odd_even_sums(10), 10);
    }

    #[test]
    fn test_n_100() {
        assert_eq!(Solution::gcd_of_odd_even_sums(100), 100);
    }
}
