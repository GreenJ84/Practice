// Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.

// Return an integer denoting the sum of all numbers in the given range satisfying the constraint.



struct Solution {}
impl Solution {
    pub fn is_multiple(n: i32) -> bool {
        if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            return true;
        }
        return false;
    }

    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if Solution::is_multiple(i) {
                ans += i;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
    }

    #[test]
    fn test_sum_of_multiples_2() {
        assert_eq!(Solution::sum_of_multiples(10), 40);
    }

    #[test]
    fn test_sum_of_multiples_3() {
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }
}