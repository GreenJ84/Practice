// Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.

// The digit sum of a positive integer is the sum of all its digits.

struct Solution;
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut ans = num / 10 * 5 - 1;
        let mut even_odd = {
            let mut sum = 0;
            let mut n = num;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum % 2
        };
        for _ in ((num / 10 * 10)..=num).rev() {
            if even_odd == 0 {
                ans += 1;
            }
            even_odd = (even_odd + 1) % 2;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_even(4), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_even(30), 14);
    }
}

// Example 1:

// Input: num = 4
// Output: 2
// Explanation:
// The only integers less than or equal to 4 whose digit sums are even are 2 and 4.
// Example 2:

// Input: num = 30
// Output: 14
// Explanation:
// The 14 integers less than or equal to 30 whose digit sums are even are
// 2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.
