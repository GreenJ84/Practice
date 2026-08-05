// Given two integers, num and t. A number x is achievable if it can become equal to num after applying the following operation at most t times:

// Increase or decrease x by 1, and simultaneously increase or decrease num by 1.
// Return the maximum possible value of x.

// Constraints:
// 1 <= num, t <= 50

struct Solution;
impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
      t * 2 + num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 4;
        let t = 1;
        let result = Solution::the_maximum_achievable_x(num, t);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let num = 3;
        let t = 2;
        let result = Solution::the_maximum_achievable_x(num, t);
        assert_eq!(result, 7);
    }
}
