// Given an integer num, return three consecutive integers (as a sorted array) that sum to num. If num cannot be expressed as the sum of three consecutive integers, return an empty array.

// Constraints:
// 0 <= num <= 10^15

struct Solution;
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 != 0 {
            return vec![];
        }
        let mid = num / 3;
        vec![mid - 1, mid, mid + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 33;
        let expected = vec![10, 11, 12];
        assert_eq!(Solution::sum_of_three(num), expected);
    }

    #[test]
    fn test_2() {
        let num = 4;
        let expected: Vec<i64> = vec![];
        assert_eq!(Solution::sum_of_three(num), expected);
    }
}
