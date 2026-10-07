// Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.

// He starts by putting in $1 on Monday, the first day. Every day from Tuesday to Sunday, he will put in $1 more than the day before. On every subsequent Monday, he will put in $1 more than the previous Monday.

// Given n, return the total amount of money he will have in the Leetcode bank at the end of the nth day.

// Constraints:
// 1 <= n <= 1000

struct Solution;
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let full_weeks: i32 = n / 7;
        let day: i32 = n % 7;

        let day_base = (0i32..full_weeks).map(|w| w + 1).sum::<i32>();
        let final_week_additional = day * (full_weeks + 1) + (0i32..day).sum::<i32>();

        7 * day_base + full_weeks * 21 + final_week_additional
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let result = Solution::total_money(n);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_2() {
        let n = 10;
        let result = Solution::total_money(n);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_3() {
        let n = 20;
        let result = Solution::total_money(n);
        assert_eq!(result, 96);
    }

    #[test]
    fn test_4() {
        let n = 1000;
        let result = Solution::total_money(n);
        assert_eq!(result, 74926);
    }

    #[test]
    fn test_5() {
        let n = 1;
        let result = Solution::total_money(n);
        assert_eq!(result, 1);
    }
}
