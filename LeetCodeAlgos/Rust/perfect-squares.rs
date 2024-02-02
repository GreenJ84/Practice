// Given an integer n, return the least number of perfect square numbers that sum to n.

// A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
use std::cmp::min;
struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![n; n as usize + 1];
        dp[0] = 0;

        for curr_step in 1..=n as usize{
            for num in 1..=curr_step {
                let square = num * num;
                if square > curr_step as usize {
                    break;
                }
                dp[curr_step] = min(dp[curr_step], dp[curr_step - square] + 1);
            }
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_num_squares2() {
        assert_eq!(Solution::num_squares(13), 2);
    }
}