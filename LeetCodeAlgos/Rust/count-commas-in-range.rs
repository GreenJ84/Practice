// You are given an integer n.

// Return the total number of commas used when writing all integers from [1, n] (inclusive) in standard number formatting.

// In standard formatting:

// A comma is inserted after every three digits from the right.
// Numbers with fewer than 4 digits contain no commas.

// Constraints:
// 1 <= n <= 10^5

struct Solution;
impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        0.max(n - 999)
    }

    pub fn count_commas1(n: i32) -> i32 {
        if n < 1000 {
            0
        } else {
            n - 999
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 1002;
        let result = Solution::count_commas1(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let n = 998;
        let result = Solution::count_commas1(n);
        assert_eq!(result, 0);
    }
}
