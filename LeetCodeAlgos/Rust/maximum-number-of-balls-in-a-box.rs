// You are working in a ball factory where you have n balls numbered from lowLimit up to highLimit inclusive (i.e., n == highLimit - lowLimit + 1), and an infinite number of boxes numbered from 1 to infinity.

// Your job at this factory is to put each ball in the box with a number equal to the sum of digits of the ball's number. For example, the ball number 321 will be put in the box number 3 + 2 + 1 = 6 and the ball number 10 will be put in the box number 1 + 0 = 1.

// Given two integers lowLimit and highLimit, return the number of balls in the box with the most balls.

// Constraints:
// 1 <= lowLimit <= highLimit <= 10^5

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut box_counts = HashMap::<i32, i32>::new();
        for mut ball in low_limit..=high_limit {
            let mut b = 0;
            while ball > 0 {
                b += ball % 10;
                ball /= 10;
            }
            *box_counts.entry(b).or_default() += 1;
        }
        box_counts.into_values().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let low_limit = 1;
        let high_limit = 10;
        assert_eq!(Solution::count_balls(low_limit, high_limit), 2);
    }

    #[test]
    fn test_2() {
        let low_limit = 5;
        let high_limit = 15;
        assert_eq!(Solution::count_balls(low_limit, high_limit), 2);
    }

    #[test]
    fn test_3() {
        let low_limit = 19;
        let high_limit = 28;
        assert_eq!(Solution::count_balls(low_limit, high_limit), 2);
    }
}
